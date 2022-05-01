use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::api::{endpoint::Endpoint, query_parameters::ExtraQueryParams};

use super::stops_response::StopPointMode;

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct QueryParams {}

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub struct StopsByModeRequest {
    modes: Vec<StopPointMode>,
    #[derivative(Default(value = "1"))]
    page: usize,
    query_params: QueryParams,
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
                .map(|m| {
                    let string = to_string(m).unwrap();
                    let mut chars = string.chars();
                    chars.next();
                    chars.next_back();
                    chars.as_str().to_string()
                })
                .collect::<Vec<String>>()
                .join(",")
        )
    }

    fn extra_query_params(&self) -> ExtraQueryParams {
        let mut params = ExtraQueryParams::new();
        params.push("page", self.page);
        params
    }

    type Parameters = QueryParams;

    fn query_params(&self) -> &Self::Parameters {
        &self.query_params
    }
}

impl StopsByModeRequest {
    pub fn new<T: Into<Vec<StopPointMode>>>(stop_point_modes: T) -> Self {
        let mut request = StopsByModeRequest::default();
        request.modes = stop_point_modes.into();
        request
    }
}
