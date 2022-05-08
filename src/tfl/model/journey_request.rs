use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::tfl::{endpoint::Endpoint, query_parameters::ExtraQueryParams};

use super::journey_response::JourneyPlannerResult;

#[derive(Serialize, Deserialize, Default)]
pub struct JourneyRequest {
    #[serde(skip)]
    pub from: String,
    #[serde(skip)]
    pub to: String,
    #[serde(skip)]
    pub datetime: Option<DateTime<Utc>>,
    pub via: Option<String>,
    pub national_search: Option<bool>,
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

impl JourneyRequest {
    pub fn new<T: Into<String>>(from: T, to: T) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            ..Default::default()
        }
    }
}

impl Endpoint for JourneyRequest {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> String {
        format!("Journey/JourneyResults/{}/to/{}", self.from, self.to)
    }

    fn extra_query_params(&self) -> ExtraQueryParams {
        let date = self.datetime.map(|val| val.date());
        let time = self.datetime.map(|val| val.time());
        let mut params = ExtraQueryParams::new();
        params.push_opt("date", date);
        params.push_opt("time", time);
        params
    }

    type Returns = JourneyPlannerResult;
}

#[cfg(test)]
mod tests {
    use crate::tfl::client::{Client, TFLClient};

    use super::*;

    #[tokio::test]
    async fn test_good_request() {
        let mut client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();

        let mut journey = JourneyRequest::new("w67qh", "sw71aa");
        journey.mode = Some("bus".into());

        let response = client.query(&journey).await;

        match response {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                panic!();
            }
        }
    }

    #[tokio::test]
    async fn test_bad_request() {
        let mut client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();

        let mut journey = JourneyRequest::new("w67qh", "sw71aa");
        journey.mode = Some("asdasdas".into());

        let response = client.query(&journey).await;

        assert!(response.is_err());
    }
}
