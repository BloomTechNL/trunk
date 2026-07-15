use crate::commit::CommitHandler;
use crate::config::{ConfigHandler, TrunkConfig};
use crate::output::OutputSink;
use crate::play_fart_sound::FartPlayer;
use crate::pull::PullHandler;
use crate::query::{DiffHandler, LogHandler, StatusHandler};
use crate::reset::ResetHandler;
use crate::revert::RevertHandler;
use crate::time_travel::TimeTravelHandler;
use crate::update::Updater;
use crate::CoAuthorAliases;

pub struct Dependencies<
    'a,
    FP: FartPlayer,
    CA: CoAuthorAliases,
    U: Updater,
    O: OutputSink,
    TC: TrunkConfig,
> {
    pub fart_player: &'a FP,
    pub co_author_aliases: &'a CA,
    pub updater: &'a U,
    pub output: &'a O,
    pub trunk_config: &'a TC,
}

pub struct HandlerContainer<'a, CA: CoAuthorAliases, TC: TrunkConfig, O: OutputSink> {
    aliases: &'a CA,
    config: &'a TC,
    sink: &'a O,
}

impl<'a, CA: CoAuthorAliases, TC: TrunkConfig, O: OutputSink> HandlerContainer<'a, CA, TC, O> {
    pub const fn new(aliases: &'a CA, config: &'a TC, sink: &'a O) -> Self {
        Self {
            aliases,
            config,
            sink,
        }
    }

    #[must_use]
    pub const fn commit(&self) -> CommitHandler<'_, CA, TC, O> {
        CommitHandler::new(self.aliases, self.config, self.sink)
    }

    #[must_use]
    pub const fn pull(&self) -> PullHandler<'_, O> {
        PullHandler::new(self.sink)
    }

    #[must_use]
    pub const fn log(&self) -> LogHandler<'_, O> {
        LogHandler::new(self.sink)
    }

    #[must_use]
    pub const fn status(&self) -> StatusHandler<'_, O> {
        StatusHandler::new(self.sink)
    }

    #[must_use]
    pub const fn diff(&self) -> DiffHandler<'_, O> {
        DiffHandler::new(self.sink)
    }

    #[must_use]
    pub const fn time_travel(&self) -> TimeTravelHandler<'_, O> {
        TimeTravelHandler::new(self.sink)
    }

    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn reset(&self) -> ResetHandler {
        ResetHandler::new()
    }

    #[must_use]
    pub const fn revert(&self) -> RevertHandler<'_, O> {
        RevertHandler::new(self.sink)
    }

    #[must_use]
    pub const fn config(&self) -> ConfigHandler<'_, TC> {
        ConfigHandler::new(self.config)
    }
}
