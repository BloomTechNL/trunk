use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::{run_cli, Cli, RealFartPlayer};

fn main() {
    let cli = Cli::parse();
    let dir = PathBuf::from(".");
    let player = RealFartPlayer;

    if let Err(e) = run_cli(cli, &dir, &player) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
