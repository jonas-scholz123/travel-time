use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::{tfl::endpoint::Endpoint, utils};

use super::stops_response::{StopsResponse, TransportMode};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub struct StopsByModeRequest {
    #[serde(skip)]
    modes: Vec<TransportMode>,
    #[derivative(Default(value = "1"))]
    pub page: usize,
}

impl Endpoint for StopsByModeRequest {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> String {
        format!(
            "StopPoint/Mode/{}/",
            self.modes
                .iter()
                .map(|m| utils::enum_to_string(m).unwrap())
                .collect::<Vec<String>>()
                .join(",")
        )
    }

    type Returns = StopsResponse;
}

impl StopsByModeRequest {
    pub fn new<T: Into<Vec<TransportMode>>>(stop_point_modes: T) -> Self {
        Self {
            modes: stop_point_modes.into(),
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tfl::client::{Client, TFLClient};

    use super::*;

    #[tokio::test]
    async fn test_request() {
        let mut client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();

        let mut request =
            StopsByModeRequest::new(vec![TransportMode::Dlr, TransportMode::CableCar]);
        request.page = 1;

        let response = client.query(&request).await;

        if response.is_err() {
            println!("{}", response.as_ref().err().unwrap())
        }

        assert!(response.is_ok());

        let stop_points = response.unwrap().stop_points;
        assert!(stop_points.is_some());
        assert!(!stop_points.unwrap().is_empty());

        request.page = 100;
        let response = client.query(&request).await;
        let stop_points = response.unwrap().stop_points;
        assert!(stop_points.is_none() || stop_points.unwrap().is_empty());
    }
}