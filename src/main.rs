use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::cli::AppService;
use g_cli::dependencies::Dependencies;
use g_cli::output::StdoutSink;
use g_cli::slot::Slot;
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

    let aliases_path = config_dir.join("aliases");
    let trunk_config_path = config_dir.join("trunk.json");
    let last_update_path = config_dir.join("last_update");

    let dependencies = Dependencies::new(
        Slot::register(|| RealFartPlayer),
        Slot::register(move || RealCoAuthorAliases::new(aliases_path)),
        Slot::register(move || {
            RealUpdater::new(RealClock, RealLastUpdateStore::new(last_update_path))
        }),
        Slot::register(|| StdoutSink),
        Slot::register(move || RealTrunkConfig::new(trunk_config_path)),
    );

    let app_service = AppService::new(dependencies);

    if let Err(e) = app_service.dispatch_command(cli, PathBuf::from(".")) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
