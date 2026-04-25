mod application;
mod domain;
mod infrastructure;

use clap::{Parser, Subcommand};
use infrastructure::{Cli, Commands};

use crate::infrastructure::setup_database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::PoolSize { pool_size } => {
            tracing::info!("Count for pool: {:?} .", pool_size);
        }
        Commands::DatabasePath { db_path } => {
            tracing::info!("Trying connect to database: {:?}", db_path);
            let path: Option<String> = db_path.clone();
            let cli_pool_size = Some(10);
            setup_database(db_path, cli_pool_size);
        }
    }
}
