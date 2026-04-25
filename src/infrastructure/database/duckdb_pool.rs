use anyhow::{Context, Result};
use duckdb::{DuckdbConnectionManager};

use crate::infrastructure::database::struct_duckdb_config::DuckDbConfig;

pub type DuckPool = r2d2::Pool<DuckdbConnectionManager>;

pub fn init_pool(options: DuckDbConfig) -> Result<DuckPool> {
    //warn: can be more than one pool but isn't necesary.
    let manager = match options.path {
        Some(p) => DuckdbConnectionManager::file(p).context("Error openening the file.")?,
        None => DuckdbConnectionManager::memory().context("Error initializing in memory.")?,
    };

    let pool = r2d2::Pool::builder()
        .max_size(options.pool_size)
        .build(manager)
        .context("Error building pool.")?;

    Ok(pool)
}
