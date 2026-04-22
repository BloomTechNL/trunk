use std::path::Path;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    cmd_commit, cmd_commit_abort, cmd_commit_resolve, cmd_diff, cmd_log, cmd_pull, cmd_reset,
    cmd_revert, cmd_revert_abort, cmd_revert_resolve, cmd_status, cmd_time_travel,
    cmd_time_travel_now, play_fart_sound::FartPlayer,
};

fn version_string() -> &'static str {
    match option_env!("GIT_HASH") {
        Some(h) => h,
        None => "unknown",
    }
}

#[derive(Parser)]
#[command(name = "g", about = "An opinionated trunk-based git adapter", version = version_string())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Commit, pull --rebase, and push.
    #[command(name = "c")]
    Commit {
        /// Commit message. Omit when using --resolve or --abort.
        message: Option<String>,
        /// Continue after resolving a rebase conflict.
        #[arg(long)]
        resolve: bool,
        /// Abort an in-progress rebase and soft-reset the local commit.
        #[arg(long)]
        abort: bool,
    },
    /// Pull (fast-forward), only allowed with a clean working directory and no unpushed commits.
    #[command(name = "p")]
    Pull,
    /// Show git log.
    #[command(name = "l")]
    Log,
    /// Show git status.
    #[command(name = "s")]
    Status,
    /// Show git diff.
    #[command(name = "d")]
    Diff,
    /// Travel to a commit by hash or relative time (e.g. "2 hours ago").
    #[command(name = "tt")]
    TimeTravel {
        /// Commit hash or relative time string.
        target: String,
    },
    /// Hard reset (git reset --hard).
    #[command(name = "r")]
    Reset,
    /// Revert a commit and sync.
    #[command(name = "rv")]
    Revert {
        /// Commit hash to revert (defaults to HEAD).
        hash: Option<String>,
        /// Continue after resolving a rebase conflict.
        #[arg(long)]
        resolve: bool,
        /// Abort -- runs git rebase --abort then git reset --hard HEAD~1.
        #[arg(long)]
        abort: bool,
    },
    /// Play a fart sound
    #[command(name = "fart")]
    Fart,
}

pub fn run_cli(cli: Cli, dir: &Path, fart_player: &dyn FartPlayer) -> Result<()> {
    // Play a fart if there's anything in the stash.
    let stash_output = std::process::Command::new("git")
        .args(["stash", "list"])
        .current_dir(dir)
        .output();
    if let Ok(out) = stash_output {
        if !out.stdout.is_empty() {
            let _ = fart_player.play_asynchronously();
        }
    }

    match cli.command {
        Commands::Commit { message, resolve, abort } => {
            if resolve {
                cmd_commit_resolve(dir)
            } else if abort {
                cmd_commit_abort(dir)
            } else if let Some(msg) = message {
                cmd_commit(dir, &msg)
            } else {
                anyhow::bail!("Usage: g c <message> | g c --resolve | g c --abort");
            }
        }
        Commands::Pull => cmd_pull(dir),
        Commands::Log => cmd_log(dir, false).map(|_| ()),
        Commands::Status => cmd_status(dir, false).map(|_| ()),
        Commands::Diff => cmd_diff(dir, false).map(|_| ()),
        Commands::TimeTravel { target } => {
            if target == "now" {
                cmd_time_travel_now(dir)
            } else {
                cmd_time_travel(dir, &target)
            }
        }
        Commands::Reset => cmd_reset(dir),
        Commands::Revert { hash, resolve, abort } => {
            if resolve {
                cmd_revert_resolve(dir)
            } else if abort {
                cmd_revert_abort(dir)
            } else {
                let h = hash.unwrap_or_else(|| "HEAD".to_string());
                cmd_revert(dir, &h, false)
            }
        }
        Commands::Fart => fart_player.play(),
    }
}
