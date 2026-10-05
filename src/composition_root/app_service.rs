use crate::cli::{Cli, Commands};
use crate::config::{RepoScopedTrunkConfig, TrunkConfig};
use crate::output::OutputSink;
use crate::play_fart_sound::FartPlayer;
use crate::update::Updater;
use crate::{run_cli, CoAuthorAliases};

use super::dependencies::Dependencies;

pub struct AppService<
    FP: FartPlayer,
    CA: CoAuthorAliases,
    U: Updater,
    O: OutputSink,
    TC: TrunkConfig,
> {
    deps: Dependencies<FP, CA, U, O, TC>,
}

impl<FP: FartPlayer, CA: CoAuthorAliases, U: Updater, O: OutputSink, TC: RepoScopedTrunkConfig>
    AppService<FP, CA, U, O, TC>
{
    #[must_use]
    pub const fn new(deps: Dependencies<FP, CA, U, O, TC>) -> Self {
        Self { deps }
    }

    pub fn dispatch_command(&self, cli: Cli, repo: &std::path::Path) -> anyhow::Result<()> {
        if !matches!(cli.command, Commands::Update) {
            self.deps.updater().auto_update(self.deps.trunk_config())?;
        }

        run_cli(cli, repo, &self.deps)
    }
}
