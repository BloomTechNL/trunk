use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::cli::AppService;
use g_cli::output::StdoutSink;
use g_cli::{Cli, RealCoAuthorAliases, RealFartPlayer, RealTrunkConfig, RealUpdater};

fn main() {
    let cli = Cli::parse();

    let alias_path = std::env::var("HOME")
        .map(PathBuf::from)
        .expect("HOME environment variable not set")
        .join(".config/trunk/aliases");

    let co_author_aliases = RealCoAuthorAliases::new(alias_path);

    let config_path = std::env::var("HOME")
        .map(PathBuf::from)
        .expect("HOME environment variable not set")
        .join(".config/trunk/trunk.json");

    let trunk_config = RealTrunkConfig::new(config_path);

    let app_service = AppService {
        fart_player: &RealFartPlayer,
        co_author_aliases: &co_author_aliases,
        trunk_config: &trunk_config,
        output: &StdoutSink,
        updater: &RealUpdater,
    };

    if let Err(e) = app_service.dispatch_command(cli, PathBuf::from(".")) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
