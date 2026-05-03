use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "Hound",
    help_template = "{about}\n\nUsage: {usage}\n\n{all-args}"
)]
#[command(author = "Jason.Gosh@github.com")]
#[command(version = "1.0")]
pub struct Cli {
    /// Text to search for; this text must be between 1 and 250 characters.
    pub target: String,
    #[arg(long)]
    /// database path: "/","/var/hound.db", etc. if not exists value then dont make file.
    #[arg(short, long)]
    pub database_path: Option<String>,
    /// config the pool size for database, must be a number between 1 and 50.
    #[arg(long)]
    pub pool_size: Option<u32>,
}
