use crate::common::capturing_sink::CapturingSink;
use crate::common::in_memory_co_author_aliases::InMemoryCoAuthorAliases;
use crate::common::in_memory_trunk_config::InMemoryTrunkConfig;
use crate::common::mock_fart_player::MockFartPlayer;
use crate::common::mock_hold_music::MockHoldMusic;
use crate::common::mock_updater::MockUpdater;
use crate::common::timeline::Timeline;
use g_cli::composition_root::{AppService, Dependencies, Slot};
use g_cli::{Cli, Commands, RepoAwareTrunkConfig};
use screenplay::Ability;
use std::path::Path;

pub struct UseTrunk {
    fart_player: MockFartPlayer,
    co_author_aliases: InMemoryCoAuthorAliases,
    trunk_config: InMemoryTrunkConfig,
    output: CapturingSink,
    updater: MockUpdater,
    hold_music: MockHoldMusic,
    timeline: Timeline,
}

impl Ability for UseTrunk {}

#[allow(dead_code)]
impl UseTrunk {
    pub fn new() -> Self {
        let timeline = Timeline::new();
        let fart_player = MockFartPlayer::new();
        let co_author_aliases = InMemoryCoAuthorAliases::new();
        let trunk_config = InMemoryTrunkConfig::new();
        let output = CapturingSink::new(timeline.clone());
        let updater = MockUpdater::new();
        let hold_music = MockHoldMusic::new(timeline.clone());
        Self {
            fart_player,
            co_author_aliases,
            trunk_config,
            output,
            updater,
            hold_music,
            timeline,
        }
    }

    pub fn dispatch(&self, command: Commands, dir: &Path) -> anyhow::Result<()> {
        self.app(dir).dispatch_command(Cli { command }, dir)
    }

    pub fn dispatch_and_capture(&self, command: Commands, dir: &Path) -> String {
        self.app(dir)
            .dispatch_command(Cli { command }, dir)
            .unwrap_or_else(|_| panic!("command should succeed"));
        self.output.take()
    }

    fn app(
        &self,
        dir: &Path,
    ) -> AppService<
        MockFartPlayer,
        InMemoryCoAuthorAliases,
        MockUpdater,
        CapturingSink,
        RepoAwareTrunkConfig<InMemoryTrunkConfig>,
        MockHoldMusic,
    > {
        let fart_player = self.fart_player.clone();
        let co_author_aliases = self.co_author_aliases.clone();
        let updater = self.updater.clone();
        let output = self.output.clone();
        let trunk_config = RepoAwareTrunkConfig::new(self.trunk_config.clone(), dir.to_path_buf());
        let hold_music = self.hold_music.clone();
        let dependencies = Dependencies::new(
            Slot::register(move || fart_player),
            Slot::register(move || co_author_aliases),
            Slot::register(move || updater),
            Slot::register(move || output),
            Slot::register(move || trunk_config),
            Slot::register(move || hold_music),
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

    pub fn hold_music_started_flag(&self) -> std::rc::Rc<std::cell::Cell<bool>> {
        self.hold_music.started_flag()
    }

    pub fn hold_music_playing_flag(&self) -> std::rc::Rc<std::cell::Cell<bool>> {
        self.hold_music.playing_flag()
    }

    pub fn timeline(&self) -> Timeline {
        self.timeline.clone()
    }
}
