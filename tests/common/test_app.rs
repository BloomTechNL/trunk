use crate::common::mock_fart_player::MockFartPlayer;
use g_cli::cli::AppService;
use g_cli::{Cli, Commands, RealCoAuthorAliases};
use std::path::Path;
use tempfile::TempDir;

pub struct TestApp {
    pub base_dir: TempDir,
    fart_player: MockFartPlayer,
}

impl TestApp {
    pub fn new() -> Self {
        let base_dir = TempDir::new().unwrap();
        let fart_player = MockFartPlayer::new();
        TestApp {
            base_dir,
            fart_player,
        }
    }

    fn app(&self) -> AppService<'_, MockFartPlayer, RealCoAuthorAliases> {
        AppService {
            fart_player: &self.fart_player,
            co_author_aliases: &RealCoAuthorAliases,
        }
    }

    pub fn was_fart_played(&self) -> bool {
        self.fart_player.was_played()
    }

    pub fn commit(&self, dir: &Path, message: &str, co_author: Option<&str>) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Commit {
                    message: Some(message.to_string()),
                    co_author: co_author.map(|s| s.to_string()),
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
                    co_author: None,
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
                    co_author: None,
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

    pub fn pull(&self, dir: &Path) -> anyhow::Result<()> {
        self.app().dispatch_command(
            Cli {
                command: Commands::Pull,
            },
            dir.to_path_buf(),
        )
    }
}
