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
    pub indicator: Option<String>,
    #[serde(rename = "stopLetter")]
    pub stop_letter: Option<String>,
    pub modes: Vec<StopPointMode>,
    #[serde(rename = "icsCode")]
    pub ics_code: Option<String>,
    #[serde(rename = "stopType")]
    pub stop_type: Option<StopType>,
    #[serde(rename = "stationNaptan")]
    pub station_naptan: Option<String>,
    pub lines: Vec<Line>,
    #[serde(rename = "lineGroup")]
    pub line_group: Vec<LineGroup>,
    #[serde(rename = "lineModeGroups")]
    pub line_mode_groups: Vec<LineModeGroup>,
    pub status: bool,
    pub id: String,
    #[serde(rename = "commonName")]
    pub common_name: String,
    #[serde(rename = "placeType")]
    pub place_type: PlaceType,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Vec<AdditionalProperty>,
    pub children: Vec<StopPoint>,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "hubNaptanCode")]
    pub hub_naptan_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalProperty {
    pub category: Category,
    pub key: Key,
    #[serde(rename = "sourceSystemKey")]
    pub source_system_key: SourceSystemKey,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGroup {
    #[serde(rename = "stationAtcoCode")]
    pub station_atco_code: String,
    #[serde(rename = "lineIdentifier")]
    pub line_identifier: Vec<String>,
    #[serde(rename = "naptanIdReference")]
    pub naptan_id_reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineModeGroup {
    #[serde(rename = "modeName")]
    pub mode_name: StopPointMode,
    #[serde(rename = "lineIdentifier")]
    pub line_identifier: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    pub id: String,
    pub name: String,
    pub uri: String,
    #[serde(rename = "type")]
    pub purple_type: Type,
    pub crowding: Crowding,
    #[serde(rename = "routeType")]
    pub route_type: RouteType,
    pub status: RouteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crowding {
    #[serde(rename = "$type")]
    pub crowding_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    Accessibility,
    Address,
    Facility,
    Geo,
    NearestPlaces,
    #[serde(rename = "Opening Time")]
    OpeningTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Key {
    AccessViaLift,
    Address,
    AddtionalInformation,
    BlueBadgeCarParkSpaces,
    #[serde(rename = "Boarding Ramps")]
    BoardingRamps,
    Bridge,
    #[serde(rename = "Car park")]
    CarPark,
    #[serde(rename = "Cash Machines")]
    CashMachines,
    Escalators,
    #[serde(rename = "Euro Cash Machines")]
    EuroCashMachines,
    Gates,
    #[serde(rename = "Help Points")]
    HelpPoints,
    Lifts,
    LimitedCapacityLift,
    MonFriFrom,
    MonFriTo,
    #[serde(rename = "Other Facilities")]
    OtherFacilities,
    Payphones,
    PhoneNo,
    #[serde(rename = "Photo Booths")]
    PhotoBooths,
    SatFrom,
    SatTo,
    SourceSystemPlaceId,
    SpecificEntranceRequired,
    SunFrom,
    SunTo,
    TaxiRankOutsideStation,
    #[serde(rename = "Ticket Halls")]
    TicketHalls,
    Toilet,
    ToiletNote,
    Toilets,
    #[serde(rename = "Waiting Room")]
    WaitingRoom,
    WiFi,
    Zone,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SourceSystemKey {
    #[serde(rename = "LRAD")]
    Lrad,
    StaticObjects,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StopPointMode {
    #[serde(rename = "bus")]
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
pub enum Type {
    Line,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RouteType {
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlaceType {
    StopPoint,
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
