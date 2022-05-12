use anyhow::{bail, Result};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use reqwest::Url;
use serde::Serialize;

use super::{endpoint::Endpoint, errors::TflBadRequest};

#[async_trait(?Send)]
pub trait Client {
    async fn query<E: Endpoint + Sync + Serialize>(&self, endpoint: &E) -> Result<E::Returns>;
    async fn query_raw<E: Endpoint + Sync + Serialize>(&self, endpoint: &E) -> Result<String>;

    async fn query_concurrently<'a, E, I>(&self, endpoints: I) -> Vec<Result<E::Returns>>
    where
        E: Endpoint + Sync + Serialize + 'a,
        I: IntoIterator<Item = &'a E> + 'a;
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

#[async_trait(?Send)]
impl Client for TFLClient {
    async fn query<E: Endpoint + Sync + Serialize>(&self, endpoint: &E) -> Result<E::Returns>
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

        let response = self.reqwest_client.execute(request).await?;
        let response_body = response.text().await?;

        // Try to decode as T.
        match serde_json::from_str(&response_body) {
            Ok(result) => {
                return Ok(result);
            }
            Err(e) => println!("Error decoding response: {}", e),
        }

        // Try to decode as server error message:
        match serde_json::from_str(&response_body) {
            Ok::<TflBadRequest, _>(failure) => bail!(
                "server error: code:\n {}\n message: {}",
                failure.http_status_code,
                failure.message
            ),
            Err(e) => bail!("{}", e),
        }
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

    async fn query_concurrently<'a, E, I>(&self, endpoints: I) -> Vec<Result<E::Returns>>
    where
        E: Endpoint + Sync + Serialize + 'a,
        I: IntoIterator<Item = &'a E> + 'a,
    {
        stream::iter(endpoints)
            .map(|e| self.query(e))
            .buffer_unordered(5)
            .collect()
            .await
    }
}
