use serde::{Deserialize, Serialize};

use crate::db::mongo_doc::MongoDoc;

use super::stops_response::{StopType, TransportMode};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimetableResult {
    pub line_id: String,
    pub line_name: String,
    pub direction: String,
    pub stations: Vec<Station>,
    pub stops: Vec<Station>,
    pub timetable: Timetable,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub id: String,
    pub station_type: Option<String>,
    pub modes: Vec<TransportMode>,
    pub stop_type: StopType,
    pub lines: Vec<Line>,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "hasDisruption")]
    pub has_disruption: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timetable {
    #[serde(rename = "departureStopId")]
    pub departure_stop_id: String,
    pub routes: Vec<Route>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    #[serde(rename = "stationIntervals")]
    pub station_intervals: Vec<StationInterval>,
    pub schedules: Vec<Schedule>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub name: String,
    pub known_journeys: Vec<Departure>,
    pub first_journey: Departure,
    pub last_journey: Departure,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    pub hour: String,
    pub minute: String,
    pub interval_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationInterval {
    pub id: String,
    pub intervals: Vec<Interval>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub stop_id: String,
    pub time_to_arrival: f64,
}
