use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Hound")]
#[command(author = "Jason.Gosh@github.com")]
#[command(version = "1.0")]
//#[command(about = "Fet tareas de forma rápida", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    
    /// database path: "/","/var/hound.db", etc. if not exists value then dont make file.
    DatabasePath{
      #[arg(alias = "db_path")]
      db_path: Option<String>,
    },
    /// config the pool for database, must be a number between 1 and 50.
    PoolSize {
        pool_size: Option<u32>,
    },
}
