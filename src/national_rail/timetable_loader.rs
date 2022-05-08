use std::{
    collections::{HashMap, HashSet},
    vec,
};

use crate::{
    db::mongo_repo::MongoRepository,
    national_rail::xml_timetable::StopType,
    tfl::model::{direct_connection::DirectConnection, stops_response::StopPoint},
};

use super::xml_timetable::TimetableDoc;
use anyhow::{bail, Result};
use chrono::NaiveTime;
use futures::{stream, StreamExt, TryStreamExt};
use mongodb::{bson::doc, Client};
use quick_xml::de::from_str;
use tokio::fs::read_to_string;

pub struct TimetableLoader {
    dc_repo: MongoRepository<DirectConnection>,
    tfl_stops_repo: MongoRepository<StopPoint>,
}
impl TimetableLoader {
    pub fn new(mongo_client: &Client) -> Self {
        Self {
            dc_repo: MongoRepository::<DirectConnection>::new(mongo_client),
            tfl_stops_repo: MongoRepository::<StopPoint>::new(mongo_client),
        }
    }
    pub async fn load_timetable<S: Into<String>>(&self, timetable_path: S) -> Result<()> {
        let xml_str = read_to_string(timetable_path.into()).await?;
        let timetable: TimetableDoc = from_str(&xml_str)?;
        // Probably should've just used a for loop... oof
        stream::iter(timetable.journeys)
            .then(|j| async move {
                let _ = stream::iter(j.stops.windows(2).map(|pair| (&pair[0], &pair[1])).filter(
                    |(prev, current)| {
                        let prev_ok = matches!(
                            prev,
                            StopType::Origin { .. } | StopType::Intermediate { .. }
                        );

                        let current_ok = matches!(
                            current,
                            StopType::Destination { .. } | StopType::Intermediate { .. }
                        );

                        prev_ok && current_ok
                    },
                ))
                .then(|(prev, current)| self.insert_connection(prev, current))
                .try_collect::<Vec<_>>()
                .await;
            })
            .collect::<Vec<_>>()
            .await;
        Ok(())
    }

    async fn insert_connection(&self, prev: &StopType, current: &StopType) -> Result<()> {
        let (depart_time, depart_tiploc) = match prev {
            StopType::Origin { departure, tiploc }
            | StopType::Intermediate {
                departure, tiploc, ..
            } => (Self::timestr_to_time(departure).unwrap(), tiploc),
            // This is wrong and should panic if it happens.
            _ => {
                panic!("Error: Trying to get departure time from invalid enum variant.")
            }
        };

        let (arrival_time, arrival_tiploc) = match current {
            // This is wrong and should panic if it happens.
            StopType::Destination { arrival, tiploc }
            | StopType::Intermediate {
                arrival, tiploc, ..
            } => (Self::timestr_to_time(arrival).unwrap(), tiploc),
            _ => panic!("Error: Trying to get arrival time from invalid enum variant."),
        };

        let mut con = DirectConnection {
            origin: self.tiploc_to_naptan(depart_tiploc).await?,
            destination: self.tiploc_to_naptan(arrival_tiploc).await?,
            duration_minutes: (arrival_time - depart_time).num_minutes() as f64,
            departure_times: vec![depart_time],
            ..Default::default()
        };
        con.mongo_insert(&self.dc_repo).await?;
        Ok(())
    }

    fn timestr_to_time(timestr: &str) -> Result<NaiveTime> {
        let time = NaiveTime::parse_from_str(&timestr[..5].to_string(), "%H:%M")?;
        Ok(time)
    }

    async fn tiploc_to_naptan(&self, tiploc: &str) -> Result<String> {
        // Look up stop points with this tiploc.
        let filter = doc! {"tiploc": tiploc.to_string()};
        let matches: Vec<_> = self
            .tfl_stops_repo
            .collection
            .find(filter, None)
            .await?
            .try_collect()
            .await?;

        match matches.iter().max_by_key(|s| s.lines.len()) {
            Some(stop) => Ok(stop.id.clone()),
            None => bail!("No match found for tiploc {}", tiploc),
        }
    }
}
