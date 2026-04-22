/// End-to-end integration tests for `g`.
///
/// Each test operates against real git repositories created in isolated
/// temporary directories.  No compiled `g` binary is invoked — the core
/// library functions are called directly.
use std::fs;
use std::path::Path;
use std::process::Command;

use g_cli::{cmd_log, cmd_revert, run_cli, Cli, Commands, FartPlayer};
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

struct MockFartPlayer;
impl FartPlayer for MockFartPlayer {
    fn play(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

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

fn g_commit(dir: &Path, message: &str) -> anyhow::Result<()> {
    run_cli(
        Cli {
            command: Commands::Commit {
                message: Some(message.to_string()),
                resolve: false,
                abort: false,
            },
        },
        dir,
        &MockFartPlayer,
    )
}

fn g_commit_resolve(dir: &Path) -> anyhow::Result<()> {
    run_cli(
        Cli {
            command: Commands::Commit {
                message: None,
                resolve: true,
                abort: false,
            },
        },
        dir,
        &MockFartPlayer,
    )
}

fn g_commit_abort(dir: &Path) -> anyhow::Result<()> {
    run_cli(
        Cli {
            command: Commands::Commit {
                message: None,
                resolve: false,
                abort: true,
            },
        },
        dir,
        &MockFartPlayer,
    )
}

fn g_pull(dir: &Path) -> anyhow::Result<()> {
    run_cli(Cli { command: Commands::Pull }, dir, &MockFartPlayer)
}

fn g_reset(dir: &Path) -> anyhow::Result<()> {
    run_cli(Cli { command: Commands::Reset }, dir, &MockFartPlayer)
}

// bypass_prompt=true so tests never hang waiting for stdin
fn g_revert(dir: &Path, hash: &str) -> anyhow::Result<()> {
    cmd_revert(dir, hash, true)
}

fn g_time_travel(dir: &Path, target: &str) -> anyhow::Result<()> {
    run_cli(
        Cli {
            command: Commands::TimeTravel {
                target: target.to_string(),
            },
        },
        dir,
        &MockFartPlayer
    )
}

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

struct Fixture {
    _tmp: TempDir,
    pub origin: std::path::PathBuf,
    pub clone_a: std::path::PathBuf,
    pub clone_b: std::path::PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let tmp = TempDir::new().unwrap();
        let origin = tmp.path().join("origin.git");
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

        Fixture { _tmp: tmp, origin, clone_a, clone_b }
    }
}

// ---------------------------------------------------------------------------
// 1.  Clean `g c` flow
// ---------------------------------------------------------------------------

#[test]
fn test_clean_commit_flow() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    write_file(dir, "hello.txt", "hello world\n");
    g_commit(dir, "add hello.txt").expect("g c should succeed");

    let log = cmd_log(dir, true).expect("g l");
    assert!(log.contains("add hello.txt"), "log should contain the commit message\n{log}");

    git(&f.clone_b, &["pull", "--rebase"]);
    let log_b = cmd_log(&f.clone_b, true).expect("g l on clone_b");
    assert!(log_b.contains("add hello.txt"), "commit should be visible from clone_b\n{log_b}");
}

// ---------------------------------------------------------------------------
// 2a.  `g p` blocked by unpushed commits
// ---------------------------------------------------------------------------

#[test]
fn test_pull_blocked_by_unpushed_commits() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    write_file(dir, "local.txt", "local only\n");
    git(dir, &["add", "."]);
    git(dir, &["commit", "-m", "local unpushed"]);

    let err = g_pull(dir).expect_err("g p should fail");
    assert!(
        err.to_string().to_lowercase().contains("unpushed"),
        "error should mention unpushed commits: {err}"
    );
}

// ---------------------------------------------------------------------------
// 2b.  `g p` blocked by uncommitted changes
// ---------------------------------------------------------------------------

#[test]
fn test_pull_blocked_by_dirty_working_dir() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    write_file(dir, "dirty.txt", "not yet committed\n");

    let err = g_pull(dir).expect_err("g p should fail with dirty workdir");
    assert!(
        err.to_string().to_lowercase().contains("uncommitted"),
        "error should mention uncommitted changes: {err}"
    );
}

// ---------------------------------------------------------------------------
// 2c.  `g p` succeeds on clean, up-to-date clone
// ---------------------------------------------------------------------------

#[test]
fn test_pull_succeeds_when_clean() {
    let f = Fixture::new();

    write_file(&f.clone_a, "new_feature.txt", "feature\n");
    g_commit(&f.clone_a, "add feature").expect("g c");
    g_pull(&f.clone_b).expect("g p should succeed");

    let log = cmd_log(&f.clone_b, true).expect("g l");
    assert!(log.contains("add feature"), "clone_b should have the new commit\n{log}");
}

// ---------------------------------------------------------------------------
// 3.  `g c` merge-conflict → resolve flow
// ---------------------------------------------------------------------------

#[test]
fn test_commit_conflict_and_resolve() {
    let f = Fixture::new();

    let shared_file = "shared.txt";
    write_file(&f.clone_a, shared_file, "version A\n");
    g_commit(&f.clone_a, "clone_a: add shared").expect("initial commit from A");

    git(&f.clone_b, &["pull", "--rebase"]);
    write_file(&f.clone_b, shared_file, "version B\n");

    write_file(&f.clone_a, shared_file, "version A2\n");
    g_commit(&f.clone_a, "clone_a: update shared").expect("second commit from A");

    let err = g_commit(&f.clone_b, "clone_b: conflicting change");
    assert!(err.is_err(), "expected conflict error");

    write_file(&f.clone_b, shared_file, "resolved content\n");
    g_commit_resolve(&f.clone_b).expect("g c --resolve should succeed");

    let log = cmd_log(&f.clone_b, true).expect("g l");
    assert!(
        log.contains("clone_b: conflicting change"),
        "resolved commit should be in the log\n{log}"
    );
}

// ---------------------------------------------------------------------------
// 3b.  `g c --abort` restores original state
// ---------------------------------------------------------------------------

#[test]
fn test_commit_conflict_and_abort() {
    let f = Fixture::new();

    let shared_file = "conflict.txt";
    write_file(&f.clone_a, shared_file, "original\n");
    g_commit(&f.clone_a, "seed conflict file").expect("seed");

    git(&f.clone_b, &["pull", "--rebase"]);

    write_file(&f.clone_a, shared_file, "clone_a update\n");
    g_commit(&f.clone_a, "clone_a update").expect("clone_a update");

    write_file(&f.clone_b, shared_file, "clone_b update\n");
    let err = g_commit(&f.clone_b, "clone_b conflicting");
    assert!(err.is_err(), "expected conflict");

    g_commit_abort(&f.clone_b).expect("g c --abort should succeed");

    let porcelain = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&f.clone_b)
        .output()
        .unwrap();
    let status_out = String::from_utf8_lossy(&porcelain.stdout);
    assert!(
        !status_out.trim().is_empty(),
        "after abort, working dir should have uncommitted changes"
    );
}

// ---------------------------------------------------------------------------
// 4.  `g rv` revert flow
// ---------------------------------------------------------------------------

#[test]
fn test_revert_flow() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    write_file(dir, "to_revert.txt", "this will be reverted\n");
    g_commit(dir, "add file to revert").expect("g c");

    let hash_output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir)
        .output()
        .unwrap();
    let commit_hash = String::from_utf8_lossy(&hash_output.stdout).trim().to_string();

    g_revert(dir, &commit_hash).expect("g rv should succeed");

    let log_after = cmd_log(dir, true).expect("g l after revert");
    assert!(
        log_after.contains("Revert") || log_after.contains("revert"),
        "a revert commit should appear in the log\n{log_after}"
    );
    assert!(
        !dir.join("to_revert.txt").exists()
            || fs::read_to_string(dir.join("to_revert.txt")).unwrap_or_default().is_empty(),
        "the reverted file should no longer have content"
    );
}

// ---------------------------------------------------------------------------
// 5.  Second `g c` while in conflict state is blocked
// ---------------------------------------------------------------------------

#[test]
fn test_commit_while_in_conflict_state_is_blocked() {
    let f = Fixture::new();

    let shared = "clash.txt";
    write_file(&f.clone_a, shared, "A\n");
    g_commit(&f.clone_a, "A init").expect("A init");

    git(&f.clone_b, &["pull", "--rebase"]);

    write_file(&f.clone_a, shared, "A updated\n");
    g_commit(&f.clone_a, "A update").expect("A update");

    write_file(&f.clone_b, shared, "B update\n");
    let _ = g_commit(&f.clone_b, "B conflicting");

    let err = g_commit(&f.clone_b, "should be blocked")
        .expect_err("should be blocked while in conflict state");
    assert!(
        err.to_string().contains("middle of resolving a conflict"),
        "unexpected error message: {err}"
    );
}

// ---------------------------------------------------------------------------
// 6.  `g c` and `g rv` with no remote tracking branch (first push)
// ---------------------------------------------------------------------------

#[test]
fn test_commit_without_remote_tracking_branch() {
    let tmp = tempfile::TempDir::new().unwrap();
    let clone = tmp.path().join("clone");

    git(tmp.path(), &["init", "--bare", "origin.git"]);
    git(tmp.path(), &["clone", "origin.git", "clone"]);
    git_config_identity(&clone);

    write_file(&clone, "first.txt", "first\n");
    g_commit(&clone, "first commit")
        .expect("g c should succeed without a remote tracking branch");

    let verify = tmp.path().join("verify");
    git(tmp.path(), &["clone", "origin.git", "verify"]);
    let log = cmd_log(&verify, true).expect("g l");
    assert!(log.contains("first commit"), "commit should have been pushed\n{log}");

    write_file(&clone, "second.txt", "second\n");
    g_commit(&clone, "second commit").expect("g c should succeed on second commit too");
}

#[test]
fn test_revert_without_remote_tracking_branch() {
    let tmp = tempfile::TempDir::new().unwrap();
    let clone = tmp.path().join("clone");

    git(tmp.path(), &["init", "--bare", "origin.git"]);
    git(tmp.path(), &["clone", "origin.git", "clone"]);
    git_config_identity(&clone);

    write_file(&clone, "a.txt", "a\n");
    g_commit(&clone, "add a").expect("first commit");

    let clone2 = tmp.path().join("clone2");
    git(tmp.path(), &["clone", "origin.git", "clone2"]);
    git_config_identity(&clone2);

    write_file(&clone2, "b.txt", "b\n");
    g_commit(&clone2, "add b").expect("clone2 first commit");

    let hash = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(&clone2)
        .output()
        .unwrap();
    let head = String::from_utf8_lossy(&hash.stdout).trim().to_string();

    g_revert(&clone2, &head).expect("g rv should succeed");

    let log = cmd_log(&clone2, true).expect("g l");
    assert!(
        log.contains("Revert") || log.contains("revert"),
        "revert commit should be in log\n{log}"
    );
}

// ---------------------------------------------------------------------------
// 8.  `g c` stages deletions (git add -A, not git add .)
// ---------------------------------------------------------------------------

#[test]
fn test_commit_stages_deleted_files() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    write_file(dir, "to_delete.txt", "goodbye\n");
    g_commit(dir, "add file that will be deleted").expect("g c seed");

    fs::remove_file(dir.join("to_delete.txt")).expect("remove file");
    g_commit(dir, "delete the file").expect("g c with deletion");

    let log = cmd_log(dir, true).expect("g l");
    assert!(log.contains("delete the file"), "deletion commit should be in log\n{log}");
    assert!(!dir.join("to_delete.txt").exists(), "deleted file should not exist after commit");

    git(&f.clone_b, &["pull", "--rebase"]);
    assert!(
        !f.clone_b.join("to_delete.txt").exists(),
        "deletion should have been pushed to the remote and visible in clone_b"
    );
}

// ---------------------------------------------------------------------------
// 7.  Time travel: detached HEAD blocks `g c` and `g rv`; `g tt now` restores
// ---------------------------------------------------------------------------

#[test]
fn test_time_travel_blocks_write_commands_and_now_restores() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    write_file(dir, "v1.txt", "v1\n");
    g_commit(dir, "v1").expect("v1");
    write_file(dir, "v2.txt", "v2\n");
    g_commit(dir, "v2").expect("v2");

    let parent_hash = {
        let out = std::process::Command::new("git")
            .args(["rev-parse", "HEAD~1"])
            .current_dir(dir)
            .output()
            .unwrap();
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };

    g_time_travel(dir, &parent_hash).expect("g tt <hash> should succeed");

    write_file(dir, "should_fail.txt", "nope\n");
    let err = g_commit(dir, "this should be blocked")
        .expect_err("g c should be blocked while time travelling");
    assert!(
        err.to_string().contains("time travelling"),
        "error should mention time travelling: {err}"
    );

    let err = cmd_revert(dir, "HEAD", true)
        .expect_err("g rv should be blocked while time travelling");
    assert!(
        err.to_string().contains("time travelling"),
        "error should mention time travelling: {err}"
    );

    g_time_travel(dir, "now").expect("g tt now should succeed");

    write_file(dir, "after_return.txt", "back\n");
    g_commit(dir, "commit after returning from time travel")
        .expect("g c should succeed after g tt now");

    let log = cmd_log(dir, true).expect("g l");
    assert!(
        log.contains("commit after returning from time travel"),
        "commit made after time travel should be in the log\n{log}"
    );
}

// ---------------------------------------------------------------------------
// 9.  `g r` hard-resets and removes untracked files/dirs
// ---------------------------------------------------------------------------

#[test]
fn test_reset_clears_tracked_and_untracked_changes() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    let subdir = dir.join("subdir");
    fs::create_dir(&subdir).unwrap();

    write_file(dir, "untracked_at_root.txt", "I should disappear\n");
    write_file(&subdir, "untracked_in_subdir.txt", "also gone\n");
    write_file(dir, "README.md", "dirty modification\n");

    g_reset(&subdir).expect("g r should succeed");

    let readme = fs::read_to_string(dir.join("README.md")).expect("README.md should exist");
    assert!(
        !readme.contains("dirty modification"),
        "README.md should have been reset\n{readme}"
    );
    assert!(
        !dir.join("untracked_at_root.txt").exists(),
        "untracked_at_root.txt should have been removed by git clean :/"
    );
    assert!(
        !subdir.join("untracked_in_subdir.txt").exists(),
        "untracked_in_subdir.txt should have been removed by git clean"
    );
}

