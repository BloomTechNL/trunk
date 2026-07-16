use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::Cli;

fn trunk_config_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .expect("HOME environment variable not set")
        .join(".config/trunk")
}

fn main() {
    let cli = Cli::parse();

    let config_dir = trunk_config_dir();
    let app_service = g_cli::composition_root::assemble(&config_dir);

    if let Err(e) = app_service.dispatch_command(cli, std::path::Path::new(".")) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
