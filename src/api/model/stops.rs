// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StopsResponse {
    #[serde(rename = "$type")]
    pub journey_planner_result_type: String,
    #[serde(rename = "stopPoints")]
    pub stop_points: Vec<StopPoint>,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub total: i64,
    pub page: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleChild {
    #[serde(rename = "$type")]
    pub child_type: String,
    #[serde(rename = "naptanId")]
    pub naptan_id: String,
    pub indicator: Option<String>,
    #[serde(rename = "stopLetter")]
    pub stop_letter: Option<String>,
    pub modes: Vec<Mode>,
    #[serde(rename = "icsCode")]
    pub ics_code: Option<String>,
    #[serde(rename = "stopType")]
    pub stop_type: Option<StopType>,
    #[serde(rename = "stationNaptan")]
    pub station_naptan: String,
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
pub struct StopPointChild {
    #[serde(rename = "$type")]
    pub child_type: String,
    #[serde(rename = "naptanId")]
    pub naptan_id: String,
    pub modes: Vec<Mode>,
    #[serde(rename = "icsCode")]
    pub ics_code: Option<String>,
    #[serde(rename = "stopType")]
    pub stop_type: StopType,
    #[serde(rename = "stationNaptan")]
    pub station_naptan: String,
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
    pub children: Vec<PurpleChild>,
    pub lat: f64,
    pub lon: f64,
    pub indicator: Option<String>,
    #[serde(rename = "stopLetter")]
    pub stop_letter: Option<String>,
    #[serde(rename = "hubNaptanCode")]
    pub hub_naptan_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopPoint {
    #[serde(rename = "$type")]
    pub stop_point_type: String,
    #[serde(rename = "naptanId")]
    pub naptan_id: String,
    pub indicator: Option<String>,
    #[serde(rename = "stopLetter")]
    pub stop_letter: Option<String>,
    pub modes: Vec<Mode>,
    #[serde(rename = "icsCode")]
    pub ics_code: Option<String>,
    #[serde(rename = "stopType")]
    pub stop_type: StopType,
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
    pub children: Vec<StopPointChild>,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "hubNaptanCode")]
    pub hub_naptan_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalProperty {
    #[serde(rename = "$type")]
    pub additional_property_type: String,
    pub category: Category,
    pub key: Key,
    #[serde(rename = "sourceSystemKey")]
    pub source_system_key: SourceSystemKey,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGroup {
    #[serde(rename = "$type")]
    pub line_group_type: String,
    #[serde(rename = "stationAtcoCode")]
    pub station_atco_code: String,
    #[serde(rename = "lineIdentifier")]
    pub line_identifier: Vec<String>,
    #[serde(rename = "naptanIdReference")]
    pub naptan_id_reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineModeGroup {
    #[serde(rename = "$type")]
    pub line_mode_group_type: String,
    #[serde(rename = "modeName")]
    pub mode_name: Mode,
    #[serde(rename = "lineIdentifier")]
    pub line_identifier: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    #[serde(rename = "$type")]
    pub line_type: String,
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
pub enum Mode {
    #[serde(rename = "bus")]
    Bus,
    #[serde(rename = "cable-car")]
    CableCar,
    #[serde(rename = "dlr")]
    Dlr,
    #[serde(rename = "international-rail")]
    InternationalRail,
    #[serde(rename = "national-rail")]
    NationalRail,
    #[serde(rename = "overground")]
    Overground,
    #[serde(rename = "plane")]
    Plane,
    #[serde(rename = "tflrail")]
    Tflrail,
    #[serde(rename = "tube")]
    Tube,
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
