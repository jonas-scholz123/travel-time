use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::db::mongo_doc::MongoDoc;

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
    pub modes: Vec<TransportMode>,
    //#[serde(rename = "icsCode")]
    //pub ics_code: Option<String>,
    pub stop_type: Option<StopType>,
    //#[serde(rename = "stationNaptan")]
    //pub station_naptan: Option<String>,
    pub lines: Vec<Line>,
    #[serde(alias = "id")]
    #[serde(rename = "_id")]
    pub id: String,
    pub common_name: String,
    //pub children: Vec<StopPoint>,
    pub lat: f64,
    pub lon: f64,
}

impl MongoDoc for StopPoint {
    fn database_name() -> &'static str {
        "tfl"
    }

    fn collection_name() -> &'static str {
        "StopPoint"
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    pub id: String,
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "kebab-case")]
pub enum TransportMode {
    Bus,
    CableCar,
    Coach,
    Cycle,
    CycleHire,
    Dlr,
    InterchangeKeepSitting,
    InterchangeSecure,
    NationalRail,
    Overground,
    ReplacementBus,
    RiverBus,
    RiverTour,
    Taxi,
    Tflrail,
    Tram,
    Tube,
    Walking,
    InternationalRail,
    Plane,
    ElizabethLine,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StopType {
    CarPickupSetDownArea,
    NaptanAirAccessArea,
    NaptanAirEntrance,
    NaptanAirportBuilding,
    NaptanBusCoachStation,
    NaptanBusWayPoint,
    NaptanCoachAccessArea,
    NaptanCoachBay,
    NaptanCoachEntrance,
    NaptanCoachServiceCoverage,
    NaptanCoachVariableBay,
    NaptanFerryAccessArea,
    NaptanFerryBerth,
    NaptanFerryEntrance,
    NaptanFerryPort,
    NaptanFlexibleZone,
    NaptanHailAndRideSection,
    NaptanLiftCableCarAccessArea,
    NaptanLiftCableCarEntrance,
    NaptanLiftCableCarStop,
    NaptanLiftCableCarStopArea,
    NaptanMarkedPoint,
    NaptanMetroAccessArea,
    NaptanMetroEntrance,
    NaptanMetroPlatform,
    NaptanMetroStation,
    NaptanOnstreetBusCoachStopCluster,
    NaptanOnstreetBusCoachStopPair,
    NaptanPrivateBusCoachTram,
    NaptanPublicBusCoachTram,
    NaptanRailAccessArea,
    NaptanRailEntrance,
    NaptanRailPlatform,
    NaptanRailStation,
    NaptanSharedTaxi,
    NaptanTaxiRank,
    NaptanUnmarkedPoint,
    TransportInterchange,
}
