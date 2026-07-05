use crate::common::capturing_sink::CapturingSink;
use crate::common::in_memory_co_author_aliases::InMemoryCoAuthorAliases;
use crate::common::in_memory_trunk_config::InMemoryTrunkConfig;
use crate::common::mock_clock::MockClock;
use crate::common::mock_fart_player::MockFartPlayer;
use crate::common::mock_last_update_store::MockLastUpdateStore;
use crate::common::mock_updater::MockUpdater;
use g_cli::cli::AppService;
use g_cli::{Cli, Commands};
use std::path::Path;
use tempfile::TempDir;

pub struct TestApp {
    pub base_dir: TempDir,
    fart_player: MockFartPlayer,
    co_author_aliases: InMemoryCoAuthorAliases,
    trunk_config: InMemoryTrunkConfig,
    output: CapturingSink,
    updater: MockUpdater,
    clock: MockClock,
    last_update_store: MockLastUpdateStore,
}

#[allow(dead_code)]
impl TestApp {
    pub fn new() -> Self {
        let base_dir = TempDir::new().unwrap();
        let fart_player = MockFartPlayer::new();
        let co_author_aliases = InMemoryCoAuthorAliases::new();
        let trunk_config = InMemoryTrunkConfig::new();
        let output = CapturingSink::new();
        let updater = MockUpdater::new();
        let clock = MockClock::new();
        let last_update_store = MockLastUpdateStore::new();
        Self {
            base_dir,
            fart_player,
            co_author_aliases,
            trunk_config,
            output,
            updater,
            clock,
            last_update_store,
        }
    }

    const fn app(
        &self,
    ) -> AppService<
        '_,
        MockFartPlayer,
        InMemoryCoAuthorAliases,
        MockUpdater,
        CapturingSink,
        InMemoryTrunkConfig,
        MockClock,
        MockLastUpdateStore,
    > {
        AppService {
            fart_player: &self.fart_player,
            co_author_aliases: &self.co_author_aliases,
            trunk_config: &self.trunk_config,
            output: &self.output,
            updater: &self.updater,
            clock: &self.clock,
            last_update_store: &self.last_update_store,
        }
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
        self.clock.inner()
    }

    pub fn add_alias(&self, alias: &str, name: &str, email: &str) -> anyhow::Result<()> {
        let path = self.base_dir.path().to_path_buf();
        self.app().dispatch_command(
            Cli {
                command: Commands::AddAlias {
                    alias: alias.to_string(),
                    name: name.to_string(),
                    email: email.to_string(),
                },
            },
            path,
        )
    }

    pub fn config(
        &self,
        dir: &Path,
        co_authors_required: Option<bool>,
        auto_update_period: Option<u64>,
    ) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Config {
                    co_authors_required,
                    auto_update_period,
                },
            },
            dir.to_path_buf(),
        )
    }

    pub fn commit(&self, dir: &Path, message: &str, co_authors: Vec<&str>) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Commit {
                    message: Some(message.to_string()),
                    co_authors: co_authors.iter().map(|s| s.to_string()).collect(),
                    resolve: false,
                    abort: false,
                },
            },
            dir.to_path_buf(),
        )
    }

    pub fn commit_resolve(&self, dir: &Path) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Commit {
                    message: None,
                    co_authors: vec![],
                    resolve: true,
                    abort: false,
                },
            },
            dir.to_path_buf(),
        )
    }

    pub fn commit_abort(&self, dir: &Path) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Commit {
                    message: None,
                    co_authors: vec![],
                    resolve: false,
                    abort: true,
                },
            },
            dir.to_path_buf(),
        )
    }

    pub fn reset(&self, dir: &Path) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Reset,
            },
            dir.to_path_buf(),
        )
    }

    pub fn fart(&self, dir: &Path) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Fart,
            },
            dir.to_path_buf(),
        )
    }

    pub fn revert(&self, dir: &Path, hash: &str) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Revert {
                    resolve: false,
                    abort: false,
                    noninteractive: true,
                    hash: Some(hash.to_string()),
                },
            },
            dir.to_path_buf(),
        )
    }

    pub fn revert_resolve(&self, dir: &Path) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Revert {
                    resolve: true,
                    abort: false,
                    noninteractive: true,
                    hash: None,
                },
            },
            dir.to_path_buf(),
        )
    }

    pub fn time_travel(&self, dir: &Path, target: &str) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::TimeTravel {
                    target: target.to_string(),
                },
            },
            dir.to_path_buf(),
        )
    }

    pub fn log(&self, dir: &Path) -> String {
        self.app()
            .dispatch_command(
                Cli {
                    command: Commands::Log,
                },
                dir.to_path_buf(),
            )
            .expect("g l should succeed");
        self.output.take()
    }

    pub fn commit_hashes(&self, dir: &Path) -> Vec<String> {
        self.log(dir)
            .lines()
            .filter(|l| l.starts_with("commit "))
            .map(|l| l.strip_prefix("commit ").unwrap().to_string())
            .collect()
    }

    pub fn status(&self, dir: &Path) -> String {
        self.app()
            .dispatch_command(
                Cli {
                    command: Commands::Status,
                },
                dir.to_path_buf(),
            )
            .expect("g s should succeed");
        self.output.take()
    }

    pub fn diff(&self, dir: &Path) -> String {
        self.app()
            .dispatch_command(
                Cli {
                    command: Commands::Diff,
                },
                dir.to_path_buf(),
            )
            .expect("g d should succeed");
        self.output.take()
    }

    pub fn pull(&self, dir: &Path) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Pull,
            },
            dir.to_path_buf(),
        )
    }

    pub fn update(&self, dir: &Path) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Update,
            },
            dir.to_path_buf(),
        )
    }
}
