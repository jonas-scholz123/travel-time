use reqwest::Method;
use serde::Serialize;

use super::query_parameters::ExtraQueryParams;

pub trait Endpoint {
    type Parameters: Serialize;

    fn method(&self) -> Method;
    fn endpoint(&self) -> String;
    fn extra_query_params(&self) -> ExtraQueryParams {
        // Parameterless queries
        ExtraQueryParams::default()
    }
    fn query_params(&self) -> &Self::Parameters;
    /*
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, &str> {
        Ok(None) // Many endpoints also do not have request bodies
    }*/
}
