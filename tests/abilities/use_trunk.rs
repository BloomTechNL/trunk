use crate::common::capturing_sink::CapturingSink;
use crate::common::in_memory_co_author_aliases::InMemoryCoAuthorAliases;
use crate::common::in_memory_trunk_config::InMemoryTrunkConfig;
use crate::common::mock_fart_player::MockFartPlayer;
use crate::common::mock_updater::MockUpdater;
use g_cli::composition_root::{AppService, Dependencies, Slot};
use g_cli::{Cli, Commands};
use screenplay::Ability;
use std::path::Path;

pub struct UseTrunk {
    fart_player: MockFartPlayer,
    co_author_aliases: InMemoryCoAuthorAliases,
    trunk_config: InMemoryTrunkConfig,
    output: CapturingSink,
    updater: MockUpdater,
}

impl Ability for UseTrunk {}

#[allow(dead_code)]
impl UseTrunk {
    pub fn new() -> Self {
        let fart_player = MockFartPlayer::new();
        let co_author_aliases = InMemoryCoAuthorAliases::new();
        let trunk_config = InMemoryTrunkConfig::new();
        let output = CapturingSink::new();
        let updater = MockUpdater::new();
        Self {
            fart_player,
            co_author_aliases,
            trunk_config,
            output,
            updater,
        }
    }

    pub fn dispatch(&self, command: Commands, dir: &Path) -> anyhow::Result<()> {
        self.app().dispatch_command(Cli { command }, dir)
    }

    pub fn dispatch_and_capture(&self, command: Commands, dir: &Path) -> String {
        self.app()
            .dispatch_command(Cli { command }, dir)
            .unwrap_or_else(|_| panic!("command should succeed"));
        self.output.take()
    }

    fn app(
        &self,
    ) -> AppService<
        MockFartPlayer,
        InMemoryCoAuthorAliases,
        MockUpdater,
        CapturingSink,
        InMemoryTrunkConfig,
    > {
        let fart_player = self.fart_player.clone();
        let co_author_aliases = self.co_author_aliases.clone();
        let updater = self.updater.clone();
        let output = self.output.clone();
        let trunk_config = self.trunk_config.clone();
        let dependencies = Dependencies::new(
            Slot::register(move || fart_player),
            Slot::register(move || co_author_aliases),
            Slot::register(move || updater),
            Slot::register(move || output),
            Slot::register(move || trunk_config),
        );
        AppService::new(dependencies)
    }

    pub fn was_fart_played(&self) -> bool {
        self.fart_player.was_played()
    }

    pub fn fart_flag(&self) -> std::rc::Rc<std::cell::Cell<bool>> {
        self.fart_player.inner()
    }

    pub fn update_count(&self) -> u32 {
        self.updater.update_count()
    }

    pub fn update_flag(&self) -> std::rc::Rc<std::cell::Cell<u32>> {
        self.updater.inner()
    }

    pub fn clock_flag(&self) -> std::rc::Rc<std::cell::Cell<u64>> {
        self.updater.clock_inner()
    }
}
