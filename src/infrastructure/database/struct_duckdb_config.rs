use std::str::FromStr;

pub struct DuckDbConfig {
    pub path: Option<String>,
    pub pool_size: u32,
}

impl DuckDbConfig {
    pub fn from_options(path: Option<String>, size: Option<u32>) -> Self {
        Self {
            path: path,
            pool_size: size.unwrap_or(10),
        }
    }
}
