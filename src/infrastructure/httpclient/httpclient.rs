use reqwest::{Client, Method, StatusCode, header::HeaderMap};
use serde::{Serialize, de::DeserializeOwned};
use std::{time::Duration};


pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
    pub timeout: u64,
}


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
    
    pub async fn send_request<R>(&self, method: Method, path: &str, headers: Option<HeaderMap>, body: Option<String>) -> Result<R, reqwest::Error> 
    where 
        R: DeserializeOwned
    {
        let url = format!("{}{}", self.base_url, path);      
        let method_clone = method.clone();
        let mut request = self.client.request(method, &url);
        tracing::info!(
            "[REQ] ({:?}) timeout: {}, url: {}",
            method_clone,
            self.timeout,
            url
        );
        if let Some(h) = headers {
            request = request.headers(h);
        }
        
        if let Some(b) = body {
            request = request.body(b);
        }
        
        request.send().await?.json::<R>().await
    }
    
  
}

