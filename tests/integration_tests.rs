use std::fs;
use std::path::{Path};
use std::process::Command;

use common::test_app::TestApp;
use common::use_git::{
    clone_repo, commit_file, put_something_in_stash, set_up_basic_repo,
    set_up_remote,
};
use g_cli::{cmd_log};
use crate::common::write_file::write_file;

mod common;

#[test]
fn test_commit_conflict_and_resolve() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let repo2 = clone_repo(app.base_dir.path(), "another_clone", "origin.git");
    let repo1 = &repo1.as_path();
    let repo2 = &repo2.as_path();

    let shared_file = "shared.txt";
    write_file(repo1, shared_file, "version A\n");
    app.commit(repo1, "clone_a: add shared")
        .expect("initial commit from A");

    app.pull(repo2).expect("Pull should succeed");
    write_file(repo2, shared_file, "version B\n");

    write_file(repo1, shared_file, "version A2\n");
    app.commit(repo1, "clone_a: update shared")
        .expect("second commit from A");

    let err = app.commit(repo2, "clone_b: conflicting change");
    assert!(err.is_err(), "expected conflict error");

    write_file(repo2, shared_file, "resolved content\n");
    app.commit_resolve(repo2)
        .expect("g c --resolve should succeed");

    let log = cmd_log(repo2, true).expect("g l");
    assert!(
        log.contains("clone_b: conflicting change"),
        "resolved commit should be in the log\n{log}"
    );
}

#[test]
fn test_commit_conflict_and_abort() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let repo2 = clone_repo(app.base_dir.path(), "another_clone", "origin.git");
    let repo1 = &repo1.as_path();
    let repo2 = &repo2.as_path();

    let shared_file = "conflict.txt";
    write_file(repo1, shared_file, "original\n");
    app.commit(repo1, "seed conflict file").expect("seed");

    app.pull(repo2).expect("Pull should succeed");

    write_file(repo1, shared_file, "clone_a update\n");
    app.commit(repo1, "clone_a update")
        .expect("clone_a update");

    write_file(repo2, shared_file, "clone_b update\n");
    let err = app.commit(repo2, "clone_b conflicting");
    assert!(err.is_err(), "expected conflict");

    app.commit_abort(repo2)
        .expect("g c --abort should succeed");

    let porcelain = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo2)
        .output()
        .unwrap();
    let status_out = String::from_utf8_lossy(&porcelain.stdout);
    assert!(
        !status_out.trim().is_empty(),
        "after abort, working dir should have uncommitted changes"
    );
}

#[test]
fn test_revert_flow() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let dir = &repo1.as_path();

    write_file(dir, "to_revert.txt", "this will be reverted\n");
    app.commit(dir, "add file to revert").expect("g c");

    let hash_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir)
        .output()
        .unwrap();
    let commit_hash = String::from_utf8_lossy(&hash_output.stdout)
        .trim()
        .to_string();

    app.revert(dir, &commit_hash).expect("g rv should succeed");

    let log_after = cmd_log(dir, true).expect("g l after revert");
    assert!(
        log_after.contains("Revert") || log_after.contains("revert"),
        "a revert commit should appear in the log\n{log_after}"
    );
    assert!(
        !dir.join("to_revert.txt").exists()
            || fs::read_to_string(dir.join("to_revert.txt"))
                .unwrap_or_default()
                .is_empty(),
        "the reverted file should no longer have content"
    );
}

#[test]
fn test_commit_while_in_conflict_state_is_blocked() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let repo2 = clone_repo(app.base_dir.path(), "another_clone", "origin.git");
    let repo1 = &repo1.as_path();
    let repo2 = &repo2.as_path();

    let shared = "clash.txt";
    write_file(repo1, shared, "A\n");
    app.commit(repo1, "A init").expect("A init");

    app.pull(repo2).expect("Pull should succeed");

    write_file(repo1, shared, "A updated\n");
    app.commit(repo1, "A update").expect("A update");

    write_file(repo2, shared, "B update\n");
    let _ = app.commit(repo2, "B conflicting");

    let err = app
        .commit(repo2, "should be blocked")
        .expect_err("should be blocked while in conflict state");
    assert!(
        err.to_string().contains("middle of resolving a conflict"),
        "unexpected error message: {err}"
    );
}

#[test]
fn test_revert_without_remote_tracking_branch() {
    let app = TestApp::new();
    let tmp = &app.base_dir;
    let origin = set_up_remote(tmp.path());
    let clone = clone_repo(tmp.path(), "clone", origin);

    write_file(&clone, "a.txt", "a\n");
    app.commit(&clone.clone().as_path(), "add a")
        .expect("first commit");

    let clone2 = clone_repo(tmp.path(), "clone2", origin);
    write_file(&clone2, "b.txt", "b\n");
    app.commit(&clone2.clone().as_path(), "add b")
        .expect("clone2 first commit");

    let hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(&clone2)
        .output()
        .unwrap();
    let head = String::from_utf8_lossy(&hash.stdout).trim().to_string();

    app.revert(&clone2.clone().as_path(), &head)
        .expect("g rv should succeed");

    let log = cmd_log(&clone2, true).expect("g l");
    assert!(
        log.contains("Revert") || log.contains("revert"),
        "revert commit should be in log\n{log}"
    );
}

#[test]
fn test_commit_stages_deleted_files() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let dir = &repo1.as_path();
    let repo2 = clone_repo(app.base_dir.path(), "another_clone", "origin.git");
    let repo2 = &repo2.as_path();

    write_file(dir, "to_delete.txt", "goodbye\n");
    app.commit(dir, "add file that will be deleted")
        .expect("g c seed");

    fs::remove_file(dir.join("to_delete.txt")).expect("remove file");
    app.commit(dir, "delete the file")
        .expect("g c with deletion");

    let log = cmd_log(dir, true).expect("g l");
    assert!(
        log.contains("delete the file"),
        "deletion commit should be in log\n{log}"
    );
    assert!(
        !dir.join("to_delete.txt").exists(),
        "deleted file should not exist after commit"
    );

    app.pull(repo2).expect("Pull should succeed");
    assert!(
        !repo2.join("to_delete.txt").exists(),
        "deletion should have been pushed to the remote and visible in clone_b"
    );
}

#[test]
fn test_time_travel_blocks_write_commands_and_now_restores() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let dir = &repo1.as_path();

    write_file(dir, "v1.txt", "v1\n");
    app.commit(dir, "v1").expect("v1");
    write_file(dir, "v2.txt", "v2\n");
    app.commit(dir, "v2").expect("v2");

    let parent_hash = {
        let out = Command::new("git")
            .args(["rev-parse", "HEAD~1"])
            .current_dir(dir)
            .output()
            .unwrap();
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };

    app.time_travel(dir, &parent_hash)
        .expect("g tt <hash> should succeed");

    write_file(dir, "should_fail.txt", "nope\n");
    let err = app
        .commit(dir, "this should be blocked")
        .expect_err("g c should be blocked while time travelling");
    assert!(
        err.to_string().contains("time travelling"),
        "error should mention time travelling: {err}"
    );

    let err = app
        .revert(dir, "HEAD")
        .expect_err("g rv should be blocked while time travelling");
    assert!(
        err.to_string().contains("time travelling"),
        "error should mention time travelling: {err}"
    );

    app.time_travel(dir, "now")
        .expect("g tt now should succeed");

    write_file(dir, "after_return.txt", "back\n");
    app.commit(dir, "commit after returning from time travel")
        .expect("g c should succeed after g tt now");

    let log = cmd_log(dir, true).expect("g l");
    assert!(
        log.contains("commit after returning from time travel"),
        "commit made after time travel should be in the log\n{log}"
    );
}

#[test]
fn test_reset_clears_tracked_and_untracked_changes() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let dir = &repo1.as_path();

    let subdir = dir.join("subdir");
    fs::create_dir(&subdir).unwrap();

    write_file(dir, "untracked_at_root.txt", "I should disappear\n");
    write_file(&subdir, "untracked_in_subdir.txt", "also gone\n");
    write_file(dir, "README.md", "dirty modification\n");

    app.reset(&subdir).expect("g r should succeed");

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

#[test]
fn test_clean_commit_flow() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let repo2 = clone_repo(app.base_dir.path(), "another_clone", "origin.git");

    write_file(repo1.as_path(), "hello.txt", "hello world\n");
    app.commit(repo1.as_path(), "add hello.txt")
        .expect("g c should succeed");

    let log = cmd_log(repo1.as_path(), true).expect("g l");
    assert!(
        log.contains("add hello.txt"),
        "log should contain the commit message\n{log}"
    );

    app.pull(repo2.as_path()).expect("Pull should succeed");
    let log_b = cmd_log(repo2.as_path(), true).expect("g l on clone_b");
    assert!(
        log_b.contains("add hello.txt"),
        "commit should be visible from clone_b\n{log_b}"
    );
}

#[test]
fn test_pull_blocked_by_unpushed_commits() {
    let app = TestApp::new();
    let repo_dir = set_up_basic_repo(app.base_dir.path());
    commit_file(repo_dir.as_path());

    let err = app.pull(repo_dir.as_path()).expect_err("should fail");

    assert!(
        err.to_string().to_lowercase().contains("unpushed"),
        "error should mention unpushed commits"
    );
}

#[test]
fn test_pull_blocked_by_dirty_working_dir() {
    let app = TestApp::new();
    let repo_dir = set_up_basic_repo(app.base_dir.path());

    write_file(repo_dir.as_path(), "dirty.txt", "not yet committed\n");

    let err = app.pull(repo_dir.as_path()).expect_err("should fail");

    assert!(
        err.to_string().to_lowercase().contains("uncommitted"),
        "error should mention uncommitted changes: {err}"
    );
}

#[test]
fn test_pull_succeeds_when_clean() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let repo2 = clone_repo(app.base_dir.path(), "another_clone", "origin.git");

    write_file(repo1.as_path(), "new_feature.txt", "feature\n");
    app.commit(repo1.as_path(), "add feature")
        .expect("g c succeeds");
    app.pull(repo2.as_path()).expect("g p succeeds");

    let log = cmd_log(repo2.as_path(), true).expect("g l");
    assert!(
        log.contains("add feature"),
        "clone2 should have the new commit\n{log}"
    );
}

#[test]
fn test_fart_plays_fart_sound() {
    let app = TestApp::new();

    app.fart(app.base_dir.path()).expect("Fart should succeed");

    assert!(app.was_fart_played(), "A fart sound should have played");
}

#[test]
fn test_fart_plays_when_stash_is_non_empty() {
    let app = TestApp::new();
    let repo_dir = set_up_basic_repo(app.base_dir.path());

    put_something_in_stash(repo_dir.as_path());

    app.pull(repo_dir.as_path()).expect("g p should succeed");

    assert!(
        app.was_fart_played(),
        "a fart should play when the stash is non-empty"
    );
}

#[test]
fn test_fart_does_not_play_when_stash_is_empty() {
    let app = TestApp::new();
    let repo_dir = set_up_basic_repo(app.base_dir.path());

    app.pull(repo_dir.as_path()).expect("g p should succeed");

    assert!(
        !app.was_fart_played(),
        "no fart should play when the stash is empty"
    );
}
