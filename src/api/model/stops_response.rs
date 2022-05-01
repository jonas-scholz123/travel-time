use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopsResponse {
    pub stop_points: Option<Vec<StopPoint>>,
    pub page_size: Option<i64>,
    pub total: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPoint {
    pub naptan_id: String,
    pub modes: Vec<StopPointMode>,
    //#[serde(rename = "icsCode")]
    //pub ics_code: Option<String>,
    pub stop_type: Option<StopType>,
    //#[serde(rename = "stationNaptan")]
    //pub station_naptan: Option<String>,
    pub lines: Vec<Line>,
    pub id: String,
    pub common_name: String,
    //pub children: Vec<StopPoint>,
    pub lat: f64,
    pub lon: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    pub id: String,
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StopPointMode {
    Bus,
    CableCar,
    Coach,
    Cycle,
    CycleHire,
    Dlr,
    ElizabethLine,
    InterchangeKeepSitting,
    InterchangeSecure,
    InternationalRail,
    NationalRail,
    Overground,
    ReplacementBus,
    RiverBus,
    RiverTour,
    Plane,
    Taxi,
    Tflrail,
    Tram,
    Tube,
    Walking,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StopType {
    NaptanBusCoachStation,
    NaptanMetroAccessArea,
    NaptanMetroEntrance,
    NaptanMetroPlatform,
    NaptanMetroStation,
    NaptanOnstreetBusCoachStopCluster,
    NaptanOnstreetBusCoachStopPair,
    NaptanPublicBusCoachTram,
    NaptanRailEntrance,
    NaptanRailStation,
    TransportInterchange,
}
