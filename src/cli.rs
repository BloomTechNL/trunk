use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commit::{commit, CommitInput};
use crate::config::{cmd_config, TrunkConfig};
use crate::output::OutputSink;
use crate::revert::{revert, RevertInput};
use crate::{
    cmd_diff, cmd_log, cmd_pull, cmd_reset, cmd_status, cmd_time_travel, has_stash,
    play_fart_sound::FartPlayer, CoAuthorAliases, Updater,
};

fn version_string() -> &'static str {
    option_env!("GIT_HASH").map_or("unknown", |h| h)
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
        /// Co-author aliases (@alias) or SOLO.
        co_authors: Vec<String>,
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
    /// Add a co-author alias
    #[command(name = "add-alias")]
    AddAlias {
        /// Alias (e.g. @piet)
        alias: String,
        /// Full name of the co-author
        #[arg(short = 'n', long)]
        name: String,
        /// Email of the co-author
        #[arg(short = 'e', long)]
        email: String,
    },
    /// Set configuration values.
    #[command(name = "config")]
    Config {
        /// Whether co-authors are required on commits. Defaults to true.
        #[arg(long = "co-authors-required")]
        co_authors_required: Option<bool>,
        /// Auto-update period in seconds. 0 disables auto-update. Defaults to 604800 (1 week).
        #[arg(long = "auto-update-period")]
        auto_update_period: Option<u64>,
    },
    /// Update g to the latest version
    #[command(name = "update")]
    Update,
}

pub fn run_cli(
    cli: Cli,
    dir: &Path,
    fart_player: &impl FartPlayer,
    aliases: &impl CoAuthorAliases,
    config: &impl TrunkConfig,
    output: &impl OutputSink,
) -> Result<()> {
    if cli.command != Commands::Fart && has_stash(dir) {
        let _ = fart_player.play_asynchronously();
    }

    match cli.command {
        Commands::Commit {
            message,
            co_authors,
            resolve,
            abort,
        } => commit(
            &CommitInput::from_cli(PathBuf::from(dir), message, co_authors, resolve, abort)?,
            aliases,
            config,
            output,
        ),
        Commands::Pull => cmd_pull(dir, output),
        Commands::Log => cmd_log(dir, output),
        Commands::Status => cmd_status(dir, output),
        Commands::Diff => cmd_diff(dir, output),
        Commands::TimeTravel { target } => cmd_time_travel(dir, &target, output),
        Commands::Reset => cmd_reset(dir),
        Commands::Revert {
            hash,
            resolve,
            abort,
            noninteractive,
        } => revert(
            &RevertInput::from_cli(PathBuf::from(dir), hash, resolve, abort, !noninteractive),
            output,
        ),
        Commands::Fart => fart_player.play(),
        Commands::FartDaemon => fart_player.run_daemon(dir),
        Commands::AddAlias { alias, name, email } => {
            let alias = alias.trim_start_matches('@');
            aliases.add_alias(alias, &name, &email)
        }
        Commands::Config {
            co_authors_required,
            auto_update_period,
        } => cmd_config(co_authors_required, auto_update_period, config),
        Commands::Update => Ok(()),
    }
}

pub struct AppService<
    'a,
    FP: FartPlayer,
    CAA: CoAuthorAliases,
    U: Updater,
    O: OutputSink,
    TC: TrunkConfig,
> {
    pub fart_player: &'a FP,
    pub co_author_aliases: &'a CAA,
    pub updater: &'a U,
    pub output: &'a O,
    pub trunk_config: &'a TC,
}

impl<'a, FP: FartPlayer, CA: CoAuthorAliases, U: Updater, O: OutputSink, TC: TrunkConfig>
    AppService<'a, FP, CA, U, O, TC>
{
    pub fn dispatch_command(&self, cli: Cli, repo: PathBuf) -> Result<()> {
        if !matches!(cli.command, Commands::Config { .. }) {
            self.updater.auto_update(self.trunk_config)?;
        }

        run_cli(
            cli,
            repo.as_path(),
            self.fart_player,
            self.co_author_aliases,
            self.trunk_config,
            self.output,
        )
    }
}
