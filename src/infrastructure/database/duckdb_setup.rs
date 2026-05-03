use super::DuckDbConfig;
use super::duckdb_pool::{DuckPool, init_pool};
use anyhow::{Context, Result};
use duckdb::{Error as DuckError};

// returns `true` if successful, `false` otherwise.
pub fn setup_database(cli_db_path: Option<String>, cli_db_pool_size: Option<u32>) -> bool {
    let dbduckconfig = DuckDbConfig::from_options(cli_db_path, cli_db_pool_size);
    let pool = match init_pool(dbduckconfig) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Cann't init pool. Cause: {:?} ", e);
            return false;
        }
    };
    match execute_setup(pool) {
        Ok(_) => {
            tracing::info!("Database infrastructure configured successfully.");
            true
        }
        Err(e) => {
            tracing::warn!(error_detail = ?e, "Database SetUp failed.");

            // Inspect if it's a specific SQL engine error
            if let Some(duck_err) = e.downcast_ref::<DuckError>() {
                log_sql_specific_error(duck_err);
            }
            false
        }
    }
}

// Internal logic to propagate errors with context using Anyhow
fn execute_setup(pool: DuckPool) -> Result<()> {
    let mut conn = pool
        .get()
        .context("Failed to acquire a connection from r2d2 pool.")?;
    let sql_content = include_str!("schema.sql");
    let tx = conn
        .transaction()
        .context("Failed to initiate database transaction.")?;
    tx.execute_batch(&sql_content)
        .context("SQL execution error in schema.sql script.")?;
    tx.commit()
        .context("Failed to commit initial database schema.")?;
    Ok(())
}

fn log_sql_specific_error(err: &DuckError) {
    match err {
        DuckError::DuckDBFailure(e, Some(msg)) => {
            tracing::warn!(
                code = ?e,
                detail = %msg,
                "The SQL engine rejected the statement."
            );
        }
        DuckError::InvalidPath(path) => {
            tracing::warn!(path = ?path, "DuckDB could not access the database file.");
        }
        _ => tracing::error!(target: "database", "Unclassified engine error: {}", err),
    }
}
