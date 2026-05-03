mod application;
mod domain;
mod infrastructure;

use clap::{Parser};
use infrastructure::Cli;
use reqwest::Method;

use crate::infrastructure::{HttpClient, setup_database};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    let _cli = Cli::parse();

    let path: Option<String> = Some("/home/jgosh/Documentos/pruebaduck/db".into());
    setup_database(path,  Some(10));
    let req = HttpClient::new("https://api.duckduckgo.com/", 20);
    match req.send_request::<serde_json::Value>(Method::GET, "?q=python&format=json&no_html=1&no_redirect=1&pretty=1", None, None).await
    {
        Ok(res) => {
            tracing::info!("data from REQ: {:?}", res);
        },
        Err(err) => {
            tracing::error!("error from REQ: {:?}", err);
        }
    }
}
