mod infrastructure;
mod orchestrator;
mod president;
mod services;

use serde::{Deserialize, Serialize};

use crate::services::httpclient::HttpClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let http = HttpClient::new("https://http.cat");

    match http.get_as_text("/status/308").await {
        Ok(texto) => tracing::info!("data: {:?}", texto),
        Err(e) => tracing::error!("Error at GET: {}", e),
    }
}
