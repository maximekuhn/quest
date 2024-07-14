use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// .http file to execute
    #[arg(short, long)]
    pub file: PathBuf,
}
