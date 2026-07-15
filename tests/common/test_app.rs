use crate::common::capturing_sink::CapturingSink;
use crate::common::in_memory_co_author_aliases::InMemoryCoAuthorAliases;
use crate::common::in_memory_trunk_config::InMemoryTrunkConfig;
use crate::common::mock_fart_player::MockFartPlayer;
use crate::common::mock_updater::MockUpdater;
use g_cli::cli::AppService;
use g_cli::container::Dependencies;
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
        Self {
            base_dir,
            fart_player,
            co_author_aliases,
            trunk_config,
            output,
            updater,
        }
    }

    fn dispatch(&self, command: Commands, dir: &Path) -> anyhow::Result<()> {
        self.app()
            .dispatch_command(Cli { command }, dir.to_path_buf())
    }

    fn dispatch_and_capture(&self, command: Commands, dir: &Path) -> String {
        self.app()
            .dispatch_command(Cli { command }, dir.to_path_buf())
            .unwrap_or_else(|_| panic!("command should succeed"));
        self.output.take()
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
    > {
        let dependencies = Dependencies {
            fart_player: &self.fart_player,
            co_author_aliases: &self.co_author_aliases,
            updater: &self.updater,
            output: &self.output,
            trunk_config: &self.trunk_config,
        };
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

    pub fn add_alias(&self, alias: &str, name: &str, email: &str) -> anyhow::Result<()> {
        let path = self.base_dir.path().to_path_buf();
        self.dispatch(
            Commands::AddAlias {
                alias: alias.to_string(),
                name: name.to_string(),
                email: email.to_string(),
            },
            &path,
        )
    }

    pub fn config(
        &self,
        dir: &Path,
        co_authors_required: Option<bool>,
        auto_update_period: Option<u64>,
    ) -> anyhow::Result<()> {
        self.dispatch(
            Commands::Config {
                co_authors_required,
                auto_update_period,
            },
            dir,
        )
    }

    pub fn commit(&self, dir: &Path, message: &str, co_authors: Vec<&str>) -> anyhow::Result<()> {
        self.dispatch(
            Commands::Commit {
                message: Some(message.to_string()),
                co_authors: co_authors.iter().map(|s| s.to_string()).collect(),
                resolve: false,
                abort: false,
            },
            dir,
        )
    }

    pub fn commit_resolve(&self, dir: &Path) -> anyhow::Result<()> {
        self.dispatch(
            Commands::Commit {
                message: None,
                co_authors: vec![],
                resolve: true,
                abort: false,
            },
            dir,
        )
    }

    pub fn commit_abort(&self, dir: &Path) -> anyhow::Result<()> {
        self.dispatch(
            Commands::Commit {
                message: None,
                co_authors: vec![],
                resolve: false,
                abort: true,
            },
            dir,
        )
    }

    pub fn reset(&self, dir: &Path) -> anyhow::Result<()> {
        self.dispatch(Commands::Reset, dir)
    }

    pub fn fart(&self, dir: &Path) -> anyhow::Result<()> {
        self.dispatch(Commands::Fart, dir)
    }

    pub fn revert(&self, dir: &Path, hash: &str) -> anyhow::Result<()> {
        self.dispatch(
            Commands::Revert {
                resolve: false,
                abort: false,
                noninteractive: true,
                hash: Some(hash.to_string()),
            },
            dir,
        )
    }

    pub fn revert_resolve(&self, dir: &Path) -> anyhow::Result<()> {
        self.dispatch(
            Commands::Revert {
                resolve: true,
                abort: false,
                noninteractive: true,
                hash: None,
            },
            dir,
        )
    }

    pub fn time_travel(&self, dir: &Path, target: &str) -> anyhow::Result<()> {
        self.dispatch(
            Commands::TimeTravel {
                target: target.to_string(),
            },
            dir,
        )
    }

    pub fn log(&self, dir: &Path) -> String {
        self.dispatch_and_capture(Commands::Log, dir)
    }

    pub fn commit_hashes(&self, dir: &Path) -> Vec<String> {
        self.log(dir)
            .lines()
            .filter(|l| l.starts_with("commit "))
            .map(|l| l.strip_prefix("commit ").unwrap().to_string())
            .collect()
    }

    pub fn status(&self, dir: &Path) -> String {
        self.dispatch_and_capture(Commands::Status, dir)
    }

    pub fn diff(&self, dir: &Path) -> String {
        self.dispatch_and_capture(Commands::Diff, dir)
    }

    pub fn pull(&self, dir: &Path) -> anyhow::Result<()> {
        self.dispatch(Commands::Pull, dir)
    }

    pub fn update(&self, dir: &Path) -> anyhow::Result<()> {
        self.dispatch(Commands::Update, dir)
    }
}
