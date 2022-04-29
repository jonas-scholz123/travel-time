use chrono::{DateTime, NaiveDate, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::api::{endpoint::Endpoint, query_parameters::QueryParameters};

#[derive(Serialize, Deserialize, Default)]
pub struct JourneyRequest {
    pub from: String,
    pub to: String,
    pub via: Option<String>,
    pub national_search: Option<bool>,
    pub datetime: Option<DateTime<Utc>>,
    pub time_is: Option<String>,
    pub journey_preference: Option<String>,
    pub mode: Option<String>,
    pub accessibility_preference: Option<String>,
    pub from_name: Option<String>,
    pub to_name: Option<String>,
    pub via_name: Option<String>,
    pub max_transfer_minutes: Option<String>,
    pub max_walking_minutes: Option<String>,
    pub walking_speed: Option<String>,
    pub cycle_preference: Option<String>,
    pub adjustment: Option<String>,
    pub bike_proficiency: Option<String>,
    pub alternative_cycle: Option<bool>,
    pub alternative_walking: Option<bool>,
    pub apply_html_markup: Option<bool>,
    pub use_multi_modal_call: Option<bool>,
    pub walking_optimization: Option<bool>,
    pub taxi_only_trip: Option<bool>,
    pub route_between_entrances: Option<bool>,
}

impl Endpoint for JourneyRequest {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> String {
        format!("Journey/JourneyResults/{}/to/{}", self.from, self.to)
    }

    fn parameters(&self) -> QueryParameters {
        let date = self.datetime.map_or(None, |val| Some(val.date()));
        let time = self.datetime.map_or(None, |val| Some(val.time()));
        let mut params = QueryParameters::new();
        params.push("from", self.from.clone());
        params.push("to", self.to.clone());
        params.push_opt("via", self.via.clone());
        params.push_opt("nationalSearch", self.national_search);
        params.push_opt("date", date);
        params.push_opt("time", time);
        params.push_opt("timeIs", self.time_is.clone());
        params.push_opt("journeyPreference", self.journey_preference.clone());
        params.push_opt("mode", self.mode.clone());
        params.push_opt(
            "accessibilityPreference",
            self.accessibility_preference.clone(),
        );
        params.push_opt("fromName", self.from_name.clone());
        params.push_opt("toName", self.to_name.clone());
        params.push_opt("viaName", self.via_name.clone());
        params.push_opt("maxTransferMinutes", self.max_transfer_minutes.clone());
        params.push_opt("maxWalkingMinutes", self.max_walking_minutes.clone());
        params.push_opt("walkingSpeed", self.walking_speed.clone());
        params.push_opt("cyclePreference", self.cycle_preference.clone());
        params.push_opt("adjustment", self.adjustment.clone());
        params.push_opt("bikeProficiency", self.bike_proficiency.clone());
        params.push_opt("alternativeCycle", self.alternative_cycle);
        params.push_opt("alternativeWalking", self.alternative_walking);
        params.push_opt("applyHtmlMarkup", self.apply_html_markup);
        params.push_opt("useMultiModalCall", self.use_multi_modal_call);
        params.push_opt("walkingOptimization", self.walking_optimization);
        params.push_opt("taxiOnlyTrip", self.taxi_only_trip);
        params.push_opt("routeBetweenEntrances", self.route_between_entrances);
        params
    }
}
