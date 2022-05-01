use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::{api::endpoint::Endpoint, utils};

use super::stops_response::StopPointMode;

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub struct StopsByModeRequest {
    #[serde(skip)]
    modes: Vec<StopPointMode>,
    #[derivative(Default(value = "1"))]
    page: usize,
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
}

impl StopsByModeRequest {
    pub fn new<T: Into<Vec<StopPointMode>>>(stop_point_modes: T) -> Self {
        let mut request = StopsByModeRequest::default();
        request.modes = stop_point_modes.into();
        request
    }
}
