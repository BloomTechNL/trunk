use crate::common::in_memory_co_author_aliases::InMemoryCoAuthorAliases;
use crate::common::mock_fart_player::MockFartPlayer;
use g_cli::cli::AppService;
use g_cli::{Cli, Commands};
use std::path::Path;
use tempfile::TempDir;

pub struct TestApp {
    pub base_dir: TempDir,
    fart_player: MockFartPlayer,
    co_author_aliases: InMemoryCoAuthorAliases,
}

impl TestApp {
    pub fn new() -> Self {
        let base_dir = TempDir::new().unwrap();
        let fart_player = MockFartPlayer::new();
        let co_author_aliases = InMemoryCoAuthorAliases::new();
        TestApp {
            base_dir,
            fart_player,
            co_author_aliases,
        }
    }

    fn app(&self) -> AppService<'_, MockFartPlayer, InMemoryCoAuthorAliases> {
        AppService {
            fart_player: &self.fart_player,
            co_author_aliases: &self.co_author_aliases,
        }
    }

    pub fn was_fart_played(&self) -> bool {
        self.fart_player.was_played()
    }

    pub fn add_alias(&mut self, alias: &str, name: &str, email: &str) -> anyhow::Result<()> {
        let content = format!("{}:{} <{}>\n", alias, name, email);
        self.co_author_aliases
            .aliases
            .insert(alias.to_string(), content);
        Ok(())
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
