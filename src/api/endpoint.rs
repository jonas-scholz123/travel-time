use reqwest::Method;

use super::query_parameters::QueryParameters;

pub trait Endpoint {
    fn method(&self) -> Method;
    fn endpoint(&self) -> String;
    fn parameters(&self) -> QueryParameters {
        // Parameterless queries
        QueryParameters::default()
    }
    /*
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, &str> {
        Ok(None) // Many endpoints also do not have request bodies
    }*/
}
