use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    /// .http file to execute
    #[arg(short, long)]
    pub file: PathBuf,

    /// environment variables file path
    #[arg(short, long, default_value = None)]
    pub dotenv: Option<PathBuf>,
}
