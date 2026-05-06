use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commit::{commit, CommitInput};
use crate::revert::{revert, RevertInput};
use crate::{
    cmd_diff, cmd_log, cmd_pull, cmd_reset, cmd_status, cmd_time_travel, has_stash,
    play_fart_sound::FartPlayer,
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

#[derive(PartialEq, Subcommand)]
pub enum Commands {
    /// Commit, pull --rebase, and push.
    #[command(name = "c")]
    Commit {
        /// Commit message.
        message: Option<String>,
        /// Co-author alias (@alias) or SOLO.
        co_author: Option<String>,
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
        /// Abort -- disables interactive mode
        #[arg(long)]
        noninteractive: bool,
    },
    /// Play a fart sound
    #[command(name = "fart")]
    Fart,
    /// Run the fart daemon (internal use)
    #[command(name = "_fart_daemon", hide = true)]
    FartDaemon,
}

pub fn run_cli(cli: Cli, dir: &Path, fart_player: &dyn FartPlayer) -> Result<()> {
    if cli.command != Commands::Fart && has_stash(dir) {
        let _ = fart_player.play_asynchronously();
    }

    match cli.command {
        Commands::Commit {
            message,
            co_author,
            resolve,
            abort,
        } => commit(&CommitInput::from_cli(
            PathBuf::from(dir),
            message,
            co_author,
            resolve,
            abort,
        )),
        Commands::Pull => cmd_pull(dir),
        Commands::Log => cmd_log(dir, false).map(|_| ()),
        Commands::Status => cmd_status(dir, false).map(|_| ()),
        Commands::Diff => cmd_diff(dir, false).map(|_| ()),
        Commands::TimeTravel { target } => cmd_time_travel(dir, &target),
        Commands::Reset => cmd_reset(dir),
        Commands::Revert {
            hash,
            resolve,
            abort,
            noninteractive,
        } => revert(&RevertInput::from_cli(
            PathBuf::from(dir),
            hash,
            resolve,
            abort,
            !noninteractive,
        )),
        Commands::Fart => fart_player.play(),
        Commands::FartDaemon => fart_player.run_daemon(dir),
    }
}

pub struct AppService<'a, FP: FartPlayer> {
    pub fart_player: &'a FP,
}

impl<'a, FP: FartPlayer> AppService<'a, FP> {
    pub fn dispatch_command(&self, cli: Cli, repo: PathBuf) -> Result<()> {
        run_cli(cli, repo.as_path(), self.fart_player)
    }
}
