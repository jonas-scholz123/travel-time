use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::{tfl::endpoint::Endpoint, utils};

use super::{
    line_response::LinesResult,
    stops_response::{StopsResponse, TransportMode},
};

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

    type Returns = LinesResult;
}

impl LinesByModeRequest {
    pub fn new<T: Into<Vec<TransportMode>>>(stop_point_modes: T) -> Self {
        Self {
            modes: stop_point_modes.into(),
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

        let request = LinesByModeRequest::new(vec![TransportMode::Dlr, TransportMode::CableCar]);

        let response = client.query(&request).await;

        if response.is_err() {
            println!("{}", response.as_ref().err().unwrap())
        }

        assert!(response.is_ok());

        let stop_points = response.unwrap();
        assert!(!stop_points.is_empty());
    }
}
