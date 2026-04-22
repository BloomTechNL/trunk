use std::path::PathBuf;
use std::process;

use clap::Parser;
use g_cli::{run_cli, Cli, RealFartPlayer, FartPlayer};

fn main() {
    // Internal subcommand used by play_asynchronously to play the fart sound in a detached process.
    if std::env::args().nth(1).as_deref() == Some("internal-fart-daemon") {
        let player = RealFartPlayer;
        let _ = player.play();
        return;
    }

    let cli = Cli::parse();
    let dir = PathBuf::from(".");
    let player = RealFartPlayer;

    if let Err(e) = run_cli(cli, &dir, &player) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
