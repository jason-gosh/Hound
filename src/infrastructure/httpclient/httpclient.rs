use reqwest::{Client, header::HeaderMap};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;

use super::HttpClient;

impl HttpClient {
    pub fn new(base_url: &str, timeout: u64) -> Self {
        Self {
            timeout: timeout,
            client: Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build()
                .unwrap(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_as_text(&self, path: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        tracing::info!(
            "[REQ] timeout: {}, base_url: {}",
            self.timeout,
            self.base_url
        );
        self.client.get(&url).send().await?.text().await
    }

    pub async fn get<R: DeserializeOwned>(&self, path: &str) -> Result<R, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        tracing::info!(
            "[REQ] timeout: {}, base_url: {}",
            self.timeout,
            self.base_url
        );
        self.client.get(&url).send().await?.json::<R>().await
    }

    pub async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
        headers: Option<HeaderMap>,
    ) -> Result<R, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.post(&url).json(body);

        if let Some(h) = headers {
            request = request.headers(h);
        }

        request.send().await?.json::<R>().await
    }

    pub async fn put<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
        headers: Option<HeaderMap>,
    ) -> Result<R, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.put(&url).json(body);

        if let Some(h) = headers {
            request = request.headers(h);
        }

        request.send().await?.json::<R>().await
    }

    pub async fn delete<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
        headers: Option<HeaderMap>,
    ) -> Result<R, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.delete(&url).json(body);

        if let Some(h) = headers {
            request = request.headers(h);
        }

        request.send().await?.json::<R>().await
    }
}
