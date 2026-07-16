use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commit::CommitInput;
use crate::config::TrunkConfig;
use crate::dependencies::Dependencies;
use crate::handler_container::HandlerContainer;
use crate::output::OutputSink;
use crate::revert::RevertInput;
use crate::{has_stash, play_fart_sound::FartPlayer, CoAuthorAliases, Handler, Updater};

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
    deps: &Dependencies<
        '_,
        impl FartPlayer,
        impl CoAuthorAliases,
        impl Updater,
        impl OutputSink,
        impl TrunkConfig,
    >,
) -> Result<()> {
    if cli.command != Commands::Fart && has_stash(dir) {
        let _ = deps.fart_player.play_asynchronously();
    }

    let container = HandlerContainer::new(deps.co_author_aliases, deps.trunk_config, deps.output);

    match cli.command {
        Commands::Commit {
            message,
            co_authors,
            resolve,
            abort,
        } => container.commit().handle(&CommitInput::from_cli(
            PathBuf::from(dir),
            message,
            co_authors,
            resolve,
            abort,
        )?),
        Commands::Pull => container.pull().handle(dir),
        Commands::Log => container.log().handle(dir),
        Commands::Status => container.status().handle(dir),
        Commands::Diff => container.diff().handle(dir),
        Commands::TimeTravel { target } => container.time_travel().handle((dir, &target)),
        Commands::Reset => container.reset().handle(dir),
        Commands::Revert {
            hash,
            resolve,
            abort,
            noninteractive,
        } => container.revert().handle(&RevertInput::from_cli(
            PathBuf::from(dir),
            hash,
            resolve,
            abort,
            !noninteractive,
        )),
        Commands::Fart => deps.fart_player.play(),
        Commands::FartDaemon => deps.fart_player.run_daemon(dir),
        Commands::AddAlias { alias, name, email } => {
            let alias = alias.trim_start_matches('@');
            deps.co_author_aliases.add_alias(alias, &name, &email)
        }
        Commands::Config {
            co_authors_required,
            auto_update_period,
        } => container
            .config()
            .handle((co_authors_required, auto_update_period)),
        Commands::Update => deps.updater.update(deps.output),
    }
}

pub struct AppService<
    'a,
    FP: FartPlayer,
    CA: CoAuthorAliases,
    U: Updater,
    O: OutputSink,
    TC: TrunkConfig,
> {
    deps: Dependencies<'a, FP, CA, U, O, TC>,
}

impl<'a, FP: FartPlayer, CA: CoAuthorAliases, U: Updater, O: OutputSink, TC: TrunkConfig>
    AppService<'a, FP, CA, U, O, TC>
{
    #[must_use]
    pub const fn new(deps: Dependencies<'a, FP, CA, U, O, TC>) -> Self {
        Self { deps }
    }

    pub fn dispatch_command(&self, cli: Cli, repo: PathBuf) -> Result<()> {
        if !matches!(cli.command, Commands::Update) {
            self.deps.updater.auto_update(self.deps.trunk_config)?;
        }

        run_cli(cli, repo.as_path(), &self.deps)
    }
}
