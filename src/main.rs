use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::{run_cli, Cli};

fn main() {
    let cli = Cli::parse();
    let dir = PathBuf::from(".");

    if let Err(e) = run_cli(cli, &dir) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
