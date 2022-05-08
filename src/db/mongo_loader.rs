use std::collections::HashMap;

use crate::tfl::{
    client::Client,
    model::{
        direct_connection::DirectConnection,
        line_request::LinesByModeRequest,
        line_response::RouteEndpoints,
        stops_request::StopsByModeRequest,
        stops_response::{StopPoint, TransportMode},
        time_table_request::TimetableRequest,
        time_table_response::TimetableResult,
    },
};
use anyhow::Result;
use chrono::{Duration, NaiveTime};
use futures::{future::join_all, TryStreamExt};
use mongodb::bson::doc;

use super::{mongo_doc::MongoDoc, mongo_repo::MongoRepository};

pub struct Loader<'a, C: Client> {
    tfl_client: &'a mut C,
    mongo_client: &'a mongodb::Client,
}

impl<'a, C: Client> Loader<'a, C> {
    pub fn new(tfl_client: &'a mut C, mongo_client: &'a mongodb::Client) -> Self {
        Self {
            tfl_client,
            mongo_client,
        }
    }

    pub fn stop_point_modes() -> Vec<TransportMode> {
        vec![
            TransportMode::Bus,
            TransportMode::CableCar,
            TransportMode::Coach,
            TransportMode::Cycle,
            TransportMode::CycleHire,
            TransportMode::Dlr,
            TransportMode::InterchangeKeepSitting,
            TransportMode::InterchangeSecure,
            TransportMode::NationalRail,
            TransportMode::Overground,
            TransportMode::ReplacementBus,
            TransportMode::RiverBus,
            TransportMode::RiverTour,
            TransportMode::Taxi,
            TransportMode::Tflrail,
            TransportMode::Tram,
            TransportMode::Tube,
            TransportMode::Walking,
        ]
    }
}

impl<'a, C: Client> Loader<'a, C> {
    pub async fn load_stops(&mut self) -> Result<()> {
        // All stop point modes.
        //let all_stop_point_modes: Vec<StopPointMode> = StopPointMode::iter().collect();

        let mut request = StopsByModeRequest::new(Loader::<'_, C>::stop_point_modes());
        let mut response = self.tfl_client.query(&request).await?;

        if let Some(total) = response.total {
            println!("Found {} matching stops", total);
        }

        let mongo_repo = MongoRepository::<StopPoint>::new(self.mongo_client);
        let existing_doc_count = mongo_repo.collection.count_documents(None, None).await?;

        // Go through page by page and transfer to Mongo.
        while let Some(stop_points) = response.stop_points {
            if stop_points.is_empty() {
                // Once the pages are empty, we're done.
                break;
            }

            println!("Fetching page {} ...", request.page);
            response = self.tfl_client.query(&request).await?;
            println!("Done fetching page {}.", request.page);

            let results = join_all(
                stop_points
                    .iter()
                    .map(|stop_point| mongo_repo.insert_or_replace(stop_point)),
            )
            .await;

            for result in results {
                if result.is_err() {
                    println!("Failure on page {}", request.page);
                    return result;
                }
            }
            request.page += 1;
        }
        let new_doc_count = mongo_repo.collection.count_documents(None, None).await?;

        println!(
            "Inserted {} documents into Mongo.",
            new_doc_count - existing_doc_count
        );

        Ok(())
    }

    pub async fn load_routes(&mut self) -> Result<()> {
        let all_modes: Vec<TransportMode> = Loader::<'_, C>::stop_point_modes();
        let request = LinesByModeRequest::new(all_modes);
        let mut lines = self.tfl_client.query(&request).await?;
        let mongo_repo = MongoRepository::<RouteEndpoints>::new(self.mongo_client);

        for line in &mut lines {
            for sec in &mut line.route_sections {
                let existing_val = mongo_repo.get_by_id(sec.id()).await?;

                // If a value exists in Mongo, we start with that one.
                if existing_val.is_some() {
                    mongo_repo
                        .collection
                        .update_one(
                            doc! {"_id": sec.id()},
                            doc! {"$addToSet": {"lineIds": line.id.clone()}},
                            None,
                        )
                        .await?;
                    break;
                }

                // Else we insert a new one.
                sec.line_ids.get_or_insert(Vec::new()).push(line.id.clone());
                sec.set_id();
                mongo_repo.insert(sec).await?;
            }
        }

        Ok(())
    }

    pub async fn load_segments(&mut self) -> Result<()> {
        let request = LinesByModeRequest::new(Loader::<'a, C>::stop_point_modes());
        let lines = self.tfl_client.query(&request).await?;
        let mongo_repo = MongoRepository::<RouteEndpoints>::new(self.mongo_client);

        let results = join_all(
            lines
                .iter()
                .flat_map(|l| l.route_sections.iter())
                .map(|sec| mongo_repo.insert_or_replace(sec)),
        )
        .await;

        for result in results {
            if result.is_err() {
                return result;
            }
        }

        Ok(())
    }

    // TODO: also reverse?
    pub async fn load_timetables(&mut self) -> Result<()> {
        let routes_repo = MongoRepository::<RouteEndpoints>::new(self.mongo_client);
        let mut cursor = routes_repo.get_all().await?;

        while let Some(route) = cursor.try_next().await? {
            // This should be for each line ID and backwards + forwards.
            let line_id = match route.line_ids {
                Some(line_ids) => line_ids.first().unwrap().clone(),
                // TOIDO Change
                None => "unknown_line".to_string(),
            };
            let origination = route.originator;
            let destination = route.destination;
            let request = TimetableRequest::new(line_id, origination, destination);
            let result = self.tfl_client.query(&request).await;

            match result {
                Ok(result) => self.save_direct_connections(result).await?,
                Err(e) => println!("{}", e),
            }
        }

        Ok(())
    }

    async fn save_direct_connections(&self, timetable: TimetableResult) -> Result<()> {
        let mut interval_id_to_journeys = HashMap::new();
        let direct_connection_repo = MongoRepository::<DirectConnection>::new(self.mongo_client);

        let origin = timetable.timetable.departure_stop_id;

        for route in &timetable.timetable.routes {
            let schedule = route.schedules.first();
            if schedule.is_none() {
                continue;
            }

            for journey in &schedule.unwrap().known_journeys {
                interval_id_to_journeys
                    .entry(journey.interval_id.to_string())
                    .or_insert_with(Vec::new)
                    .push(journey);
            }

            for interval in &route.station_intervals {
                let journeys = interval_id_to_journeys.get(&interval.id);
                if journeys.is_none() {
                    println!("Error: missing interval {}.", interval.id);
                    continue;
                }
                let journeys = journeys.unwrap();
                //let departure_times =

                let mut total_time_travelled = 0 as f64;
                let mut current_stop = origin.clone();
                for section in &interval.intervals {
                    let destination = section.stop_id.clone();
                    let minutes_between_stations = section.time_to_arrival - total_time_travelled;

                    let mut departure_times = journeys
                        .iter()
                        .map(|j| {
                            let hr_int = j.hour.parse::<u32>().unwrap() % 24;
                            let min_int = j.minute.parse::<u32>().unwrap();
                            NaiveTime::from_hms(hr_int, min_int, 0)
                                + Duration::minutes(total_time_travelled as i64)
                        })
                        .collect::<Vec<NaiveTime>>();
                    total_time_travelled = section.time_to_arrival;

                    departure_times.sort();
                    departure_times.dedup();

                    let mut direct_connection = DirectConnection {
                        id: None,
                        origin: current_stop.clone(),
                        destination: destination.clone(),
                        duration_minutes: minutes_between_stations,
                        departure_times,
                    };
                    direct_connection.set_id();
                    current_stop = destination.clone();

                    if direct_connection_repo
                        .get_by_id(direct_connection.id())
                        .await?
                        .is_none()
                    {
                        direct_connection_repo.insert(&direct_connection).await?;
                        continue;
                    }

                    // Otherwise, we already have this pair in Mongo and simply add to it.
                    let departure_time_strings = direct_connection
                        .departure_times
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>();

                    direct_connection_repo
                        .collection
                        .update_one(
                            doc! {"_id": direct_connection.id()},
                            doc! {"$addToSet": {"departure_times": {"$each": departure_time_strings }}},
                            None,
                        )
                        .await?;
                }
            }
        }

        Ok(())
    }
}
