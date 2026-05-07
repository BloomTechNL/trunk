use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::cli::AppService;
use g_cli::{Cli, RealCoAuthorAliases, RealFartPlayer};

fn main() {
    let cli = Cli::parse();

    let home = std::env::var("HOME").map(PathBuf::from).or_else(|_| {
        std::env::var("USERPROFILE").map(PathBuf::from) // Windows support just in case
    });

    let alias_path = if let Ok(home_path) = home {
        home_path.join(".config/trunk/aliases")
    } else {
        PathBuf::from(".config/trunk/aliases")
    };

    let co_author_aliases = RealCoAuthorAliases::new(alias_path);

    let app_service = AppService {
        fart_player: &RealFartPlayer,
        co_author_aliases: &co_author_aliases,
    };

    if let Err(e) = app_service.dispatch_command(cli, PathBuf::from(".")) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
