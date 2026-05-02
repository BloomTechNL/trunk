use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::{Cli, RealFartPlayer};
use g_cli::cli::AppService;

fn main() {
    let cli = Cli::parse();

    let app_service = AppService {
        fart_player: &RealFartPlayer,
    };

    if let Err(e) = app_service.dispatch_command(cli, PathBuf::from(".")) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
