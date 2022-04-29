use futures::executor;
use std::error::Error; // 0.3.1

use async_trait::async_trait;
use reqwest::{RequestBuilder, Url};
use serde::{de::DeserializeOwned, Serialize};

use super::endpoint::Endpoint;

pub trait Client {
    fn query<T: DeserializeOwned, E: Endpoint + Sync + Serialize>(
        &self,
        endpoint: &E,
    ) -> Result<T, Box<dyn Error>>;

    fn query_raw<E: Endpoint + Sync + Serialize>(
        &self,
        endpoint: &E,
    ) -> Result<String, Box<dyn Error>>;
}

pub struct TFLClient {
    base_url: Url,
    reqwest_client: reqwest::blocking::Client,
    api_key: String,
}

impl TFLClient {
    pub fn new(api_key: &str) -> Result<Self, url::ParseError> {
        let url = Url::parse("https://api.tfl.gov.uk/")?;
        Ok(TFLClient {
            base_url: url,
            reqwest_client: reqwest::blocking::Client::new(),
            api_key: api_key.into(),
        })
    }
}

impl Client for TFLClient {
    fn query_raw<E>(&self, endpoint: &E) -> Result<String, Box<dyn Error>>
    where
        E: Endpoint + Sync + Serialize,
    {
        let joined_url = self.base_url.join(&endpoint.endpoint())?;
        let response = self
            .reqwest_client
            .request(endpoint.method(), joined_url)
            .query(&[("app_key", &self.api_key)])
            .query(endpoint)
            .send()?
            //.await?
            //.json::<T>()?;
            .text()?;
        //.await?;
        Ok(response)
    }

    fn query<T: DeserializeOwned, E: Endpoint + Sync + Serialize>(
        &self,
        endpoint: &E,
    ) -> Result<T, Box<dyn Error>>
    where
        E: Endpoint + Sync + Serialize,
    {
        let joined_url = self.base_url.join(&endpoint.endpoint())?;
        let response = self
            .reqwest_client
            .request(endpoint.method(), joined_url)
            .query(&[("app_key", &self.api_key)])
            .query(endpoint)
            .send()?
            //.await?
            .json::<T>()?;
        //.await?;
        Ok(response)
    }
}
