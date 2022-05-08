use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::{tfl::endpoint::Endpoint, utils};

use super::stops_response::TransportMode;

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub struct LinesByModeRequest {
    #[serde(skip)]
    modes: Vec<TransportMode>,
}

impl Endpoint for LinesByModeRequest {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> String {
        format!(
            "Line/Mode/{}/Route",
            self.modes
                .iter()
                .map(|m| utils::enum_to_string(m).unwrap())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl LinesByModeRequest {
    pub fn new<T: Into<Vec<TransportMode>>>(stop_point_modes: T) -> Self {
        let mut request = LinesByModeRequest::default();
        request.modes = stop_point_modes.into();
        request
    }
}

#[cfg(test)]
mod tests {
    use crate::tfl::{
        client::{Client, TFLClient},
        model::stops_response::StopsResponse,
    };

    use super::*;

    #[tokio::test]
    async fn test_request() {
        let mut client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();

        let mut request =
            LinesByModeRequest::new(vec![TransportMode::Dlr, TransportMode::CableCar]);

        let response = client.query::<StopsResponse, _>(&request).await;

        if response.is_err() {
            println!("{}", response.as_ref().err().unwrap())
        }

        assert!(response.is_ok());

        let stop_points = response.unwrap().stop_points;
        assert!(stop_points.is_some());
        assert!(!stop_points.unwrap().is_empty());

        let response = client.query::<StopsResponse, _>(&request).await;
        let stop_points = response.unwrap().stop_points;
        assert!(stop_points.is_none() || stop_points.unwrap().is_empty());
    }
}
