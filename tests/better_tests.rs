use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use g_cli::cli::AppService;
use g_cli::{cmd_log, Cli, Commands, FartPlayer};
use tempfile::TempDir;
// ---------------------------------------------------------------------------
// In-memory test adapter
// ---------------------------------------------------------------------------

struct MockFartPlayer {
    played: Cell<bool>,
}

impl MockFartPlayer {
    fn new() -> Self {
        Self {
            played: Cell::new(false),
        }
    }
    fn was_played(&self) -> bool {
        self.played.get()
    }
}

impl FartPlayer for MockFartPlayer {
    fn play(&self) -> anyhow::Result<()> {
        self.played.set(true);
        Ok(())
    }

    fn play_asynchronously(&self) -> anyhow::Result<()> {
        self.played.set(true);
        Ok(())
    }

    fn run_daemon(&self, dir: &Path) -> anyhow::Result<()> {
        g_cli::run_fart_daemon(self, dir)
    }
}

// ---------------------------------------------------------------------------
// Git helpers
// ---------------------------------------------------------------------------

fn git_config_identity(dir: &Path) {
    for (k, v) in &[
        ("user.email", "test@example.com"),
        ("user.name", "Test User"),
        ("commit.gpgsign", "false"),
        ("rebase.autostash", "false"),
    ] {
        Command::new("git")
            .args(["config", k, v])
            .current_dir(dir)
            .status()
            .expect("git config");
    }
}

fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_EDITOR", "true")
        .env("GIT_TERMINAL_PROMPT", "0")
        .status()
        .expect("git command failed");
    assert!(status.success(), "git {} failed", args.join(" "));
}

fn write_file(dir: &Path, name: &str, content: &str) {
    fs::write(dir.join(name), content).expect("write file");
}

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

struct Fixture {
    _tmp: TempDir,
    pub clone_a: PathBuf,
    pub clone_b: PathBuf,
    player: MockFartPlayer,
    // We can't store AppService here because it would borrow self.player.
}

impl Fixture {
    fn new() -> Self {
        let tmp = TempDir::new().unwrap();
        let clone_a = tmp.path().join("clone_a");
        let clone_b = tmp.path().join("clone_b");

        git(tmp.path(), &["init", "--bare", "origin.git"]);
        git(tmp.path(), &["clone", "origin.git", "clone_a"]);
        git_config_identity(&clone_a);
        write_file(&clone_a, "README.md", "# project\n");
        git(&clone_a, &["add", "."]);
        git(&clone_a, &["commit", "-m", "init"]);
        git(&clone_a, &["push"]);
        git(tmp.path(), &["clone", "origin.git", "clone_b"]);
        git_config_identity(&clone_b);

        let player = MockFartPlayer::new();

        Fixture {
            _tmp: tmp,
            clone_a,
            clone_b,
            player,
        }
    }

    fn app(&self) -> AppService<'_, MockFartPlayer> {
        AppService {
            fart_player: &self.player,
        }
    }

    fn was_fart_played(&self) -> bool {
        self.player.was_played()
    }
}

#[test]
fn test_clean_commit_flow() {
    let f = Fixture::new();

    write_file(&f.clone_a, "hello.txt", "hello world\n");
    f.app()
        .dispatch_command(
            Cli {
                command: Commands::Commit {
                    message: Some("add hello.txt".to_string()),
                    resolve: false,
                    abort: false,
                },
            },
            f.clone_a.clone(),
        )
        .expect("g c should succeed");

    let log = cmd_log(&f.clone_a, true).expect("g l");
    assert!(
        log.contains("add hello.txt"),
        "log should contain the commit message\n{log}"
    );

    git(&f.clone_b, &["pull", "--rebase"]);
    let log_b = cmd_log(&f.clone_b, true).expect("g l on clone_b");
    assert!(
        log_b.contains("add hello.txt"),
        "commit should be visible from clone_b\n{log_b}"
    );
}

#[test]
fn test_pull_blocked_by_unpushed_commits() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    write_file(dir, "local.txt", "local only\n");
    git(dir, &["add", "."]);
    git(dir, &["commit", "-m", "local unpushed"]);

    let err = f
        .app()
        .dispatch_command(
            Cli {
                command: Commands::Pull,
            },
            PathBuf::from(dir),
        )
        .expect_err("should fail");

    assert!(
        err.to_string().to_lowercase().contains("unpushed"),
        "error should mention unpushed commits"
    );
}

#[test]
fn test_pull_blocked_by_dirty_working_dir() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    write_file(dir, "dirty.txt", "not yet committed\n");

    let err = f
        .app()
        .dispatch_command(
            Cli {
                command: Commands::Pull,
            },
            PathBuf::from(dir),
        )
        .expect_err("should fail");

    assert!(
        err.to_string().to_lowercase().contains("uncommitted"),
        "error should mention uncommitted changes: {err}"
    );
}

#[test]
fn test_pull_succeeds_when_clean() {
    let f = Fixture::new();

    write_file(&f.clone_a, "new_feature.txt", "feature\n");
    f.app()
        .dispatch_command(
            Cli {
                command: Commands::Commit {
                    message: Some(String::from("add feature")),
                    resolve: false,
                    abort: false,
                },
            },
            PathBuf::from(&f.clone_a),
        )
        .expect("g c succeeds");
    f.app()
        .dispatch_command(
            Cli {
                command: Commands::Pull,
            },
            PathBuf::from(&f.clone_b),
        )
        .expect("g p succeeds");

    let log = cmd_log(&f.clone_b, true).expect("g l");
    assert!(
        log.contains("add feature"),
        "clone_b should have the new commit\n{log}"
    );
}

#[test]
fn test_fart_plays_fart_sound() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    f.app()
        .dispatch_command(
            Cli {
                command: Commands::Fart,
            },
            PathBuf::from(dir),
        )
        .expect("Fart should succeed");

    assert!(f.was_fart_played(), "A fart sound should have played",);
}

#[test]
fn test_fart_plays_when_stash_is_non_empty() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    write_file(dir, "stashed.txt", "stash me\n");
    git(dir, &["add", "."]);
    git(dir, &["stash"]);

    f.app()
        .dispatch_command(
            Cli {
                command: Commands::Pull,
            },
            PathBuf::from(dir),
        )
        .expect("g p should succeed");

    assert!(
        f.was_fart_played(),
        "a fart should play when the stash is non-empty"
    );
}

#[test]
fn test_fart_does_not_play_when_stash_is_empty() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    f.app()
        .dispatch_command(
            Cli {
                command: Commands::Pull,
            },
            PathBuf::from(dir),
        )
        .expect("g p should succeed");

    assert!(
        !f.was_fart_played(),
        "no fart should play when the stash is empty"
    );
}
