use super::{DuckDbConfig, duckdb_pool};

pub fn setup_database(cli_path: Option<String>, cli_pool_size: Option<u32>) {
    let duckdb_options = DuckDbConfig::from_options(cli_path,cli_pool_size);
    match duckdb_pool::init_pool(duckdb_options) {
        Ok(pool) => {
            tracing::debug!("Pool state: {:?}", pool.state());
        }
        Err(e) => {
            tracing::error!("Cann't init pool cause: {:?}", e);
        }
    }
}
