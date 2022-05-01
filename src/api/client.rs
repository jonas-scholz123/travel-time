use anyhow::Result;
use async_trait::async_trait;
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};

use super::endpoint::Endpoint;

#[async_trait]
pub trait Client {
    async fn query<T: DeserializeOwned, E: Endpoint + Sync + Serialize>(
        &self,
        endpoint: &E,
    ) -> Result<T>;

    /*
    async fn query_paged<T: DeserializeOwned, E: Endpoint + Sync + Serialize>(
        &self,
        endpoint: &Paged<E>,
    ) -> Result<T>;
    */

    async fn query_raw<E: Endpoint + Sync + Serialize>(&self, endpoint: &E) -> Result<String>;
}

pub struct TFLClient {
    base_url: Url,
    reqwest_client: reqwest::Client,
    api_key: String,
}

impl TFLClient {
    pub fn new(api_key: &str) -> Result<Self, url::ParseError> {
        let url = Url::parse("https://api.tfl.gov.uk/")?;
        Ok(TFLClient {
            base_url: url,
            reqwest_client: reqwest::Client::new(),
            api_key: api_key.into(),
        })
    }
}

#[async_trait]
impl Client for TFLClient {
    async fn query<T: DeserializeOwned, E: Endpoint + Sync + Serialize>(
        &self,
        endpoint: &E,
    ) -> Result<T>
    where
        E: Endpoint + Sync + Serialize,
    {
        let joined_url = self.base_url.join(&endpoint.endpoint())?;

        let request = self
            .reqwest_client
            .request(endpoint.method(), joined_url.clone())
            .query(&[("app_key", &self.api_key)])
            .query(&endpoint)
            .query(&endpoint.extra_query_params())
            .build()?;

        let response = self
            .reqwest_client
            .execute(request)
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }

    async fn query_raw<E>(&self, endpoint: &E) -> anyhow::Result<String>
    where
        E: Endpoint + Sync + Serialize,
    {
        let joined_url = self.base_url.join(&endpoint.endpoint())?;
        let response = self
            .reqwest_client
            .request(endpoint.method(), joined_url)
            .query(&[("app_key", &self.api_key)])
            .query(endpoint)
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }
}
