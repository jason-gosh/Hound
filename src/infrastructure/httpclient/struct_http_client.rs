use reqwest::Client;

pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
    pub timeout: u64,
}
