use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StopsResponse {
    #[serde(rename = "stopPoints")]
    pub stop_points: Option<Vec<StopPoint>>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,
    pub total: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopPoint {
    #[serde(rename = "naptanId")]
    pub naptan_id: String,
    pub modes: Vec<StopPointMode>,
    //#[serde(rename = "icsCode")]
    //pub ics_code: Option<String>,
    #[serde(rename = "stopType")]
    pub stop_type: Option<StopType>,
    //#[serde(rename = "stationNaptan")]
    //pub station_naptan: Option<String>,
    pub lines: Vec<Line>,
    pub id: String,
    #[serde(rename = "commonName")]
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
//#[serde(rename_all = "kebab-case")]
pub enum StopPointMode {
    Bus,
    #[serde(rename = "cable-car")]
    CableCar,
    #[serde(rename = "coach")]
    Coach,
    #[serde(rename = "cycle")]
    Cycle,
    #[serde(rename = "cycle-hire")]
    CycleHire,
    #[serde(rename = "dlr")]
    Dlr,
    #[serde(rename = "elizabeth-line")]
    ElizabethLine,
    #[serde(rename = "interchange-keep-sitting")]
    InterchangeKeepSitting,
    #[serde(rename = "interchange-secure")]
    InterchangeSecure,
    #[serde(rename = "international-rail")]
    InternationalRail,
    #[serde(rename = "national-rail")]
    NationalRail,
    #[serde(rename = "overground")]
    Overground,
    #[serde(rename = "replacement-bus")]
    ReplacementBus,
    #[serde(rename = "river-bus")]
    RiverBus,
    #[serde(rename = "river-tour")]
    RiverTour,
    #[serde(rename = "plane")]
    Plane,
    #[serde(rename = "taxi")]
    Taxi,
    #[serde(rename = "tflrail")]
    Tflrail,
    #[serde(rename = "tram")]
    Tram,
    #[serde(rename = "tube")]
    Tube,
    #[serde(rename = "walking")]
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
