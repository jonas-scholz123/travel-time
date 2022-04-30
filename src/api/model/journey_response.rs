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
pub struct JourneyPlannerResult {
    #[serde(rename = "$type")]
    pub journey_planner_result_type: String,
    pub journeys: Vec<Journey>,
    pub lines: Vec<Line>,
    #[serde(rename = "stopMessages")]
    pub stop_messages: Vec<Option<serde_json::Value>>,
    #[serde(rename = "recommendedMaxAgeMinutes")]
    pub recommended_max_age_minutes: i64,
    #[serde(rename = "searchCriteria")]
    pub search_criteria: SearchCriteria,
    #[serde(rename = "journeyVector")]
    pub journey_vector: JourneyVector,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyVector {
    #[serde(rename = "$type")]
    pub journey_vector_type: String,
    pub from: String,
    pub to: String,
    pub via: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Journey {
    #[serde(rename = "$type")]
    pub journey_type: String,
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,
    pub duration: i64,
    #[serde(rename = "arrivalDateTime")]
    pub arrival_date_time: String,
    pub legs: Vec<Leg>,
    pub fare: Option<JourneyFare>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyFare {
    #[serde(rename = "$type")]
    pub fare_type: String,
    #[serde(rename = "totalCost")]
    pub total_cost: i64,
    pub fares: Vec<FareElement>,
    pub caveats: Vec<Caveat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Caveat {
    #[serde(rename = "$type")]
    pub caveat_type: String,
    pub text: String,
    #[serde(rename = "type")]
    pub purple_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FareElement {
    #[serde(rename = "$type")]
    pub fare_type: String,
    #[serde(rename = "lowZone")]
    pub low_zone: i64,
    #[serde(rename = "highZone")]
    pub high_zone: i64,
    pub cost: i64,
    #[serde(rename = "isHopperFare")]
    pub is_hopper_fare: bool,
    pub peak: i64,
    #[serde(rename = "offPeak")]
    pub off_peak: i64,
    pub taps: Vec<Tap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tap {
    #[serde(rename = "$type")]
    pub tap_type: String,
    #[serde(rename = "atcoCode")]
    pub atco_code: String,
    #[serde(rename = "tapDetails")]
    pub tap_details: TapDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TapDetails {
    #[serde(rename = "$type")]
    pub tap_details_type: String,
    #[serde(rename = "modeType")]
    pub mode_type: String,
    #[serde(rename = "validationType")]
    pub validation_type: String,
    #[serde(rename = "hostDeviceType")]
    pub host_device_type: String,
    #[serde(rename = "busRouteId")]
    pub bus_route_id: String,
    #[serde(rename = "nationalLocationCode")]
    pub national_location_code: i64,
    #[serde(rename = "tapTimestamp")]
    pub tap_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leg {
    #[serde(rename = "$type")]
    pub leg_type: String,
    pub duration: i64,
    pub instruction: Instruction,
    pub obstacles: Vec<Option<serde_json::Value>>,
    #[serde(rename = "departureTime")]
    pub departure_time: String,
    #[serde(rename = "arrivalTime")]
    pub arrival_time: String,
    #[serde(rename = "departurePoint")]
    pub departure_point: Point,
    #[serde(rename = "arrivalPoint")]
    pub arrival_point: Point,
    pub path: Path,
    #[serde(rename = "routeOptions")]
    pub route_options: Vec<RouteOption>,
    pub mode: Mode,
    pub disruptions: Vec<DisruptionElement>,
    #[serde(rename = "plannedWorks")]
    pub planned_works: Vec<Option<serde_json::Value>>,
    pub distance: Option<f64>,
    #[serde(rename = "isDisrupted")]
    pub is_disrupted: bool,
    #[serde(rename = "hasFixedLocations")]
    pub has_fixed_locations: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    #[serde(rename = "$type")]
    pub point_type: String,
    #[serde(rename = "naptanId")]
    pub naptan_id: Option<String>,
    #[serde(rename = "platformName")]
    pub platform_name: Option<String>,
    #[serde(rename = "stopLetter")]
    pub stop_letter: Option<String>,
    #[serde(rename = "icsCode")]
    pub ics_code: Option<String>,
    #[serde(rename = "commonName")]
    pub common_name: String,
    #[serde(rename = "placeType")]
    pub place_type: String,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Vec<Option<serde_json::Value>>>,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisruptionElement {
    #[serde(rename = "$type")]
    pub disruption_type: String,
    pub category: String,
    #[serde(rename = "type")]
    pub purple_type: String,
    #[serde(rename = "categoryDescription")]
    pub category_description: String,
    pub description: String,
    pub summary: String,
    #[serde(rename = "additionalInfo")]
    pub additional_info: String,
    pub created: String,
    #[serde(rename = "lastUpdate")]
    pub last_update: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    #[serde(rename = "$type")]
    pub instruction_type: String,
    pub summary: String,
    pub detailed: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    #[serde(rename = "$type")]
    pub step_type: String,
    pub description: String,
    #[serde(rename = "turnDirection")]
    pub turn_direction: String,
    #[serde(rename = "streetName")]
    pub street_name: String,
    pub distance: i64,
    #[serde(rename = "cumulativeDistance")]
    pub cumulative_distance: i64,
    #[serde(rename = "skyDirection")]
    pub sky_direction: i64,
    #[serde(rename = "skyDirectionDescription")]
    pub sky_direction_description: String,
    #[serde(rename = "cumulativeTravelTime")]
    pub cumulative_travel_time: i64,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "pathAttribute")]
    pub path_attribute: Crowding,
    #[serde(rename = "descriptionHeading")]
    pub description_heading: String,
    #[serde(rename = "trackType")]
    pub track_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crowding {
    #[serde(rename = "$type")]
    pub crowding_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mode {
    #[serde(rename = "$type")]
    pub mode_type: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub purple_type: String,
    #[serde(rename = "routeType")]
    pub route_type: String,
    pub status: String,
    pub uri: Option<String>,
    pub crowding: Option<Crowding>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
    #[serde(rename = "$type")]
    pub path_type: String,
    #[serde(rename = "lineString")]
    pub line_string: String,
    #[serde(rename = "stopPoints")]
    pub stop_points: Vec<Mode>,
    pub elevation: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteOption {
    #[serde(rename = "$type")]
    pub route_option_type: String,
    pub name: String,
    pub directions: Vec<String>,
    #[serde(rename = "lineIdentifier")]
    pub line_identifier: Option<Mode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    #[serde(rename = "$type")]
    pub line_type: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "modeName")]
    pub mode_name: String,
    pub disruptions: Vec<Option<serde_json::Value>>,
    pub created: String,
    pub modified: String,
    #[serde(rename = "lineStatuses")]
    pub line_statuses: Vec<LineStatus>,
    #[serde(rename = "routeSections")]
    pub route_sections: Vec<Option<serde_json::Value>>,
    #[serde(rename = "serviceTypes")]
    pub service_types: Vec<ServiceType>,
    pub crowding: Crowding,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineStatus {
    #[serde(rename = "$type")]
    pub line_status_type: String,
    pub id: i64,
    #[serde(rename = "statusSeverity")]
    pub status_severity: i64,
    #[serde(rename = "statusSeverityDescription")]
    pub status_severity_description: String,
    pub created: String,
    #[serde(rename = "validityPeriods")]
    pub validity_periods: Vec<ValidityPeriod>,
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    pub reason: Option<String>,
    pub disruption: Option<LineStatusDisruption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineStatusDisruption {
    #[serde(rename = "$type")]
    pub disruption_type: String,
    pub category: String,
    #[serde(rename = "categoryDescription")]
    pub category_description: String,
    pub description: String,
    pub created: String,
    #[serde(rename = "affectedRoutes")]
    pub affected_routes: Vec<Option<serde_json::Value>>,
    #[serde(rename = "affectedStops")]
    pub affected_stops: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityPeriod {
    #[serde(rename = "$type")]
    pub validity_period_type: String,
    #[serde(rename = "fromDate")]
    pub from_date: String,
    #[serde(rename = "toDate")]
    pub to_date: String,
    #[serde(rename = "isNow")]
    pub is_now: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(rename = "$type")]
    pub service_type_type: String,
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCriteria {
    #[serde(rename = "$type")]
    pub search_criteria_type: String,
    #[serde(rename = "dateTime")]
    pub date_time: String,
    #[serde(rename = "dateTimeType")]
    pub date_time_type: String,
    #[serde(rename = "timeAdjustments")]
    pub time_adjustments: TimeAdjustments,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeAdjustments {
    #[serde(rename = "$type")]
    pub time_adjustments_type: String,
    pub earliest: Earlier,
    pub earlier: Earlier,
    pub later: Earlier,
    pub latest: Earlier,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Earlier {
    #[serde(rename = "$type")]
    pub earlier_type: String,
    pub date: String,
    pub time: String,
    #[serde(rename = "timeIs")]
    pub time_is: String,
    pub uri: String,
}
