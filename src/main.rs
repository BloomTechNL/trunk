use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::cli::AppService;
use g_cli::output::StdoutSink;
use g_cli::{
    Cli, RealClock, RealCoAuthorAliases, RealFartPlayer, RealLastUpdateStore, RealTrunkConfig,
    RealUpdater,
};

fn trunk_config_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .expect("HOME environment variable not set")
        .join(".config/trunk")
}

fn main() {
    let cli = Cli::parse();

    let config_dir = trunk_config_dir();

    let co_author_aliases = RealCoAuthorAliases::new(config_dir.join("aliases"));

    let trunk_config = RealTrunkConfig::new(config_dir.join("trunk.json"));

    let last_update_store = RealLastUpdateStore::new(config_dir.join("last_update"));
    let updater = RealUpdater::new(RealClock, last_update_store);

    let app_service = AppService {
        fart_player: &RealFartPlayer,
        co_author_aliases: &co_author_aliases,
        trunk_config: &trunk_config,
        output: &StdoutSink,
        updater: &updater,
    };

    if let Err(e) = app_service.dispatch_command(cli, PathBuf::from(".")) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
