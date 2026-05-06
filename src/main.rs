use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::cli::AppService;
use g_cli::{Cli, RealCoAuthorAliases, RealFartPlayer};

fn main() {
    let cli = Cli::parse();

    let app_service = AppService {
        fart_player: &RealFartPlayer,
        co_author_aliases: &RealCoAuthorAliases,
    };

    if let Err(e) = app_service.dispatch_command(cli, PathBuf::from(".")) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
