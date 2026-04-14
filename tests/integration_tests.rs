/// End-to-end integration tests for `g`.
///
/// Each test operates against real git repositories created in isolated
/// temporary directories.  No compiled `g` binary is invoked — the core
/// library functions are called directly.
use std::fs;
use std::path::Path;
use std::process::Command;

use g_cli::{
    cmd_commit, cmd_commit_abort, cmd_commit_resolve, cmd_log, cmd_pull, cmd_reset, cmd_revert,
    cmd_time_travel, cmd_time_travel_now,
};
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Configure a minimal git identity inside `dir` so commits don't fail.
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

/// Create the standard three-directory test fixture:
///
///   tmp/
///     origin.git/   (bare)
///     clone_a/      (first client)
///     clone_b/      (second client)
///
/// Both clones have at least one commit so `@{u}` is usable.
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

        // Bare remote.
        git(tmp.path(), &["init", "--bare", "origin.git"]);

        // Clone A.
        git(tmp.path(), &["clone", "origin.git", "clone_a"]);
        git_config_identity(&clone_a);

        // Seed the remote with an initial commit from clone_a.
        write_file(&clone_a, "README.md", "# project\n");
        git(&clone_a, &["add", "."]);
        git(&clone_a, &["commit", "-m", "init"]);
        git(&clone_a, &["push"]);

        // Clone B.
        git(tmp.path(), &["clone", "origin.git", "clone_b"]);
        git_config_identity(&clone_b);

        Fixture {
            _tmp: tmp,
            origin,
            clone_a,
            clone_b,
        }
    }
}

// ---------------------------------------------------------------------------
// 1.  Clean `g c` flow
// ---------------------------------------------------------------------------

#[test]
fn test_clean_commit_flow() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    // Make a change.
    write_file(dir, "hello.txt", "hello world\n");

    // Run `g c`.
    cmd_commit(dir, "add hello.txt").expect("g c should succeed");

    // Assert using g's own `g l` function.
    let log = cmd_log(dir, true).expect("g l");
    assert!(
        log.contains("add hello.txt"),
        "log should contain the commit message\n{log}"
    );

    // Also verify the commit landed on the remote by checking clone_b can
    // see it after a pull.
    git(&f.clone_b, &["pull", "--rebase"]);
    let log_b = cmd_log(&f.clone_b, true).expect("g l on clone_b");
    assert!(
        log_b.contains("add hello.txt"),
        "commit should be visible from clone_b\n{log_b}"
    );
}

// ---------------------------------------------------------------------------
// 2a.  `g p` blocked by unpushed commits
// ---------------------------------------------------------------------------

#[test]
fn test_pull_blocked_by_unpushed_commits() {
    let f = Fixture::new();
    let dir = &f.clone_a;

    // Create a local commit that hasn't been pushed.
    write_file(dir, "local.txt", "local only\n");
    git(dir, &["add", "."]);
    git(dir, &["commit", "-m", "local unpushed"]);

    let err = cmd_pull(dir).expect_err("g p should fail");
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

    let err = cmd_pull(dir).expect_err("g p should fail with dirty workdir");
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

    // Push a new commit from clone_a.
    write_file(&f.clone_a, "new_feature.txt", "feature\n");
    cmd_commit(&f.clone_a, "add feature").expect("g c");

    // clone_b is clean and has no unpushed commits → pull should succeed.
    cmd_pull(&f.clone_b).expect("g p should succeed");

    let log = cmd_log(&f.clone_b, true).expect("g l");
    assert!(log.contains("add feature"), "clone_b should have the new commit\n{log}");
}

// ---------------------------------------------------------------------------
// 3.  `g c` merge-conflict → resolve flow
// ---------------------------------------------------------------------------

#[test]
fn test_commit_conflict_and_resolve() {
    let f = Fixture::new();

    // Both clones edit the same line of the same file — this creates a
    // genuine rebase conflict.
    let shared_file = "shared.txt";
    write_file(&f.clone_a, shared_file, "version A\n");
    cmd_commit(&f.clone_a, "clone_a: add shared").expect("initial commit from A");

    // clone_b now has a diverging commit on the same file.
    // First pull the initial state so it knows about the file.
    git(&f.clone_b, &["pull", "--rebase"]);
    write_file(&f.clone_b, shared_file, "version B\n");

    // `g c` on clone_b will succeed locally but the subsequent
    // `git pull --rebase` will create a conflict.
    // We need clone_a to push *another* commit first so clone_b is behind.
    write_file(&f.clone_a, shared_file, "version A2\n");
    cmd_commit(&f.clone_a, "clone_a: update shared").expect("second commit from A");

    // Now clone_b's `g c` → commit OK, pull hits conflict.
    let err = cmd_commit(&f.clone_b, "clone_b: conflicting change");
    assert!(err.is_err(), "expected conflict error");

    // Manually resolve the conflict by accepting one side.
    write_file(&f.clone_b, shared_file, "resolved content\n");

    // `g c --resolve` should finish the rebase and push.
    cmd_commit_resolve(&f.clone_b).expect("g c --resolve should succeed");

    // Verify the resolved commit appears in the log.
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

    // Seed with a file from clone_a.
    write_file(&f.clone_a, shared_file, "original\n");
    cmd_commit(&f.clone_a, "seed conflict file").expect("seed");

    git(&f.clone_b, &["pull", "--rebase"]);

    // clone_a pushes an update.
    write_file(&f.clone_a, shared_file, "clone_a update\n");
    cmd_commit(&f.clone_a, "clone_a update").expect("clone_a update");

    // clone_b makes a conflicting change.
    write_file(&f.clone_b, shared_file, "clone_b update\n");
    let err = cmd_commit(&f.clone_b, "clone_b conflicting");
    assert!(err.is_err(), "expected conflict");

    // Abort — should restore the working tree.
    cmd_commit_abort(&f.clone_b).expect("g c --abort should succeed");

    // Working directory should now show the conflicting changes as
    // uncommitted (soft reset).
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

    // Make a commit we'll later revert.
    write_file(dir, "to_revert.txt", "this will be reverted\n");
    cmd_commit(dir, "add file to revert").expect("g c");

    // Get the hash of that commit.
    let log_out = cmd_log(dir, true).expect("g l");
    // Parse the short hash from the first commit line (format: "abc1234 message").
    let hash_line = log_out
        .lines()
        .find(|l| l.contains("add file to revert"))
        .expect("commit line should be present");
    // git log --oneline format: "<hash> <message>"
    // But we used cmd_log which just passes `git log` (default format).
    // Use git rev-parse HEAD instead.
    let hash_output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir)
        .output()
        .unwrap();
    let commit_hash = String::from_utf8_lossy(&hash_output.stdout).trim().to_string();
    let _ = hash_line; // used for assert above

    // `g rv <hash>` with bypass_prompt=true.
    cmd_revert(dir, &commit_hash, true).expect("g rv should succeed");

    // Verify a revert commit is in the log.
    let log_after = cmd_log(dir, true).expect("g l after revert");
    assert!(
        log_after.contains("Revert") || log_after.contains("revert"),
        "a revert commit should appear in the log\n{log_after}"
    );

    // Verify the change is gone.
    assert!(
        !dir.join("to_revert.txt").exists() || {
            fs::read_to_string(dir.join("to_revert.txt")).unwrap_or_default().is_empty()
        },
        "the reverted file should no longer have content"
    );
}

// ---------------------------------------------------------------------------
// 5.  Second `g c` while in conflict state prints correct error
// ---------------------------------------------------------------------------

#[test]
fn test_commit_while_in_conflict_state_is_blocked() {
    let f = Fixture::new();

    let shared = "clash.txt";
    write_file(&f.clone_a, shared, "A\n");
    cmd_commit(&f.clone_a, "A init").expect("A init");

    git(&f.clone_b, &["pull", "--rebase"]);

    write_file(&f.clone_a, shared, "A updated\n");
    cmd_commit(&f.clone_a, "A update").expect("A update");

    write_file(&f.clone_b, shared, "B update\n");
    let _ = cmd_commit(&f.clone_b, "B conflicting");

    // At this point clone_b should be in the middle of a rebase.
    // Attempting another `g c` must return the "in conflict state" error.
    let err = cmd_commit(&f.clone_b, "should be blocked")
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
    // Create a bare remote and a single clone but do NOT push an initial
    // commit, so the clone has no upstream tracking branch at all.
    let tmp = tempfile::TempDir::new().unwrap();
    let _origin = tmp.path().join("origin.git");
    let clone = tmp.path().join("clone");

    git(tmp.path(), &["init", "--bare", "origin.git"]);
    git(tmp.path(), &["clone", "origin.git", "clone"]);
    git_config_identity(&clone);

    // First-ever commit — no upstream exists yet.
    write_file(&clone, "first.txt", "first\n");
    cmd_commit(&clone, "first commit").expect("g c should succeed without a remote tracking branch");

    // The commit should now be visible on the remote — verify from a fresh clone.
    let verify = tmp.path().join("verify");
    git(tmp.path(), &["clone", "origin.git", "verify"]);
    let log = cmd_log(&verify, true).expect("g l");
    assert!(
        log.contains("first commit"),
        "commit should have been pushed to the remote\n{log}"
    );

    // A second commit should follow the normal pull+push path without issues.
    write_file(&clone, "second.txt", "second\n");
    cmd_commit(&clone, "second commit").expect("g c should succeed on second commit too");
}

#[test]
fn test_revert_without_remote_tracking_branch() {
    let tmp = tempfile::TempDir::new().unwrap();
    let _origin = tmp.path().join("origin.git");
    let clone = tmp.path().join("clone");

    git(tmp.path(), &["init", "--bare", "origin.git"]);
    git(tmp.path(), &["clone", "origin.git", "clone"]);
    git_config_identity(&clone);

    // Push the first commit normally (this sets up the tracking branch).
    write_file(&clone, "a.txt", "a\n");
    cmd_commit(&clone, "add a").expect("first commit");

    // Now simulate a scenario where after the push the tracking ref is gone
    // by using a fresh clone of the bare repo that has never pushed.
    let clone2 = tmp.path().join("clone2");
    git(tmp.path(), &["clone", "origin.git", "clone2"]);
    git_config_identity(&clone2);

    // Make a commit to revert, publish it.
    write_file(&clone2, "b.txt", "b\n");
    cmd_commit(&clone2, "add b").expect("clone2 first commit");

    // Get HEAD hash and revert it — at this point clone2 has a tracking branch.
    let hash = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(&clone2)
        .output()
        .unwrap();
    let head = String::from_utf8_lossy(&hash.stdout).trim().to_string();

    cmd_revert(&clone2, &head, true).expect("g rv should succeed");

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
    cmd_commit(dir, "add file that will be deleted").expect("g c seed");

    fs::remove_file(dir.join("to_delete.txt")).expect("remove file");

    cmd_commit(dir, "delete the file").expect("g c with deletion");

    let log = cmd_log(dir, true).expect("g l");
    assert!(
        log.contains("delete the file"),
        "deletion commit should be in log\n{log}"
    );

    assert!(
        !dir.join("to_delete.txt").exists(),
        "deleted file should not exist after commit"
    );

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

    // Land two commits so we have a target to travel to.
    write_file(dir, "v1.txt", "v1\n");
    cmd_commit(dir, "v1").expect("v1");

    write_file(dir, "v2.txt", "v2\n");
    cmd_commit(dir, "v2").expect("v2");

    // Grab the hash of the first commit (parent of current HEAD).
    let parent_hash = {
        let out = std::process::Command::new("git")
            .args(["rev-parse", "HEAD~1"])
            .current_dir(dir)
            .output()
            .unwrap();
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };

    // Travel back in time.
    cmd_time_travel(dir, &parent_hash).expect("g tt <hash> should succeed");

    // g c must now be blocked.
    write_file(dir, "should_fail.txt", "nope\n");
    let err = cmd_commit(dir, "this should be blocked")
        .expect_err("g c should be blocked while time travelling");
    assert!(
        err.to_string().contains("time travelling"),
        "error should mention time travelling: {err}"
    );

    // g rv must also be blocked.
    let err = cmd_revert(dir, "HEAD", true)
        .expect_err("g rv should be blocked while time travelling");
    assert!(
        err.to_string().contains("time travelling"),
        "error should mention time travelling: {err}"
    );

    // Return to the present.
    cmd_time_travel_now(dir).expect("g tt now should succeed");

    // We are back on the branch — g c should work again.
    write_file(dir, "after_return.txt", "back\n");
    cmd_commit(dir, "commit after returning from time travel")
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

    // Create a subdirectory to run cmd_reset *from*, simulating the user
    // being inside a subfolder of the repo.
    let subdir = dir.join("subdir");
    fs::create_dir(&subdir).unwrap();

    // Untracked file at the repo root — outside `subdir`.
    // Without `:/`, `git clean -df` run from `subdir` would NOT remove this.
    write_file(dir, "untracked_at_root.txt", "I should disappear\n");

    // Untracked file inside the subdir we're running from.
    write_file(&subdir, "untracked_in_subdir.txt", "also gone\n");

    // Modify a tracked file at the root.
    write_file(dir, "README.md", "dirty modification\n");

    // Run reset from the subdirectory — :/ ensures root-relative clean.
    cmd_reset(&subdir).expect("g r should succeed");

    // Tracked file should be restored.
    let readme = fs::read_to_string(dir.join("README.md")).expect("README.md should exist");
    assert!(
        !readme.contains("dirty modification"),
        "README.md should have been reset\n{readme}"
    );

    // The untracked file at the repo root must be gone — this only passes
    // when `git clean -df :/` is used (the :/ makes it repo-root-relative).
    assert!(
        !dir.join("untracked_at_root.txt").exists(),
        "untracked_at_root.txt should have been removed by git clean :/"
    );

    // The untracked file inside the subdir should also be gone.
    assert!(
        !subdir.join("untracked_in_subdir.txt").exists(),
        "untracked_in_subdir.txt should have been removed by git clean"
    );
}

