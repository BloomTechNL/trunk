use crate::config::TrunkConfig;
use crate::output::OutputSink;
use crate::play_fart_sound::FartPlayer;
use crate::update::Updater;
use crate::CoAuthorAliases;

use super::slot::Slot;

pub struct Dependencies<
    FP: FartPlayer,
    CA: CoAuthorAliases,
    U: Updater,
    O: OutputSink,
    TC: TrunkConfig,
> {
    fart_player: Slot<FP>,
    co_author_aliases: Slot<CA>,
    updater: Slot<U>,
    output: Slot<O>,
    trunk_config: Slot<TC>,
}

impl<FP: FartPlayer, CA: CoAuthorAliases, U: Updater, O: OutputSink, TC: TrunkConfig>
    Dependencies<FP, CA, U, O, TC>
{
    #[must_use]
    pub const fn new(
        fart_player: Slot<FP>,
        co_author_aliases: Slot<CA>,
        updater: Slot<U>,
        output: Slot<O>,
        trunk_config: Slot<TC>,
    ) -> Self {
        Self {
            fart_player,
            co_author_aliases,
            updater,
            output,
            trunk_config,
        }
    }

    pub fn fart_player(&self) -> &FP {
        self.fart_player.resolve()
    }

    pub fn co_author_aliases(&self) -> &CA {
        self.co_author_aliases.resolve()
    }

    pub fn updater(&self) -> &U {
        self.updater.resolve()
    }

    pub fn output(&self) -> &O {
        self.output.resolve()
    }

    pub fn trunk_config(&self) -> &TC {
        self.trunk_config.resolve()
    }
}
