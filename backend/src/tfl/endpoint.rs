use reqwest::Method;
use serde::de::DeserializeOwned;

pub trait Endpoint {
    type Returns: DeserializeOwned + Send + Sync;
    fn method(&self) -> Method;
    fn endpoint(&self) -> String;
}
