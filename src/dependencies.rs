use crate::config::TrunkConfig;
use crate::output::OutputSink;
use crate::play_fart_sound::FartPlayer;
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
