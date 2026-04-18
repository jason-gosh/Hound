use reqwest::{Client, StatusCode, header::HeaderMap};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            base_url: base_url.to_string(),
        }
    }
    
    pub async fn get_as_text(&self, path: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        self.client.get(&url).send().await?.text().await
    }

    pub async fn get<R: DeserializeOwned>(&self, path: &str) -> Result<R, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
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
