use crate::common::write_file::write_file;
use common::test_app::TestApp;
use common::use_git::{clone_repo, put_something_in_stash, set_up_basic_repo};

mod common;

#[test]
fn test_time_travel_blocks_write_commands_and_now_restores() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let dir = &repo1.as_path();

    write_file(dir, "v1.txt", "v1\n");
    app.commit(dir, "v1", vec!["SOLO"]).expect("v1");
    write_file(dir, "v2.txt", "v2\n");
    app.commit(dir, "v2", vec!["SOLO"]).expect("v2");

    let parent_hash = &app.commit_hashes(dir)[1];

    app.time_travel(dir, &parent_hash)
        .expect("g tt <hash> should succeed");

    write_file(dir, "should_fail.txt", "nope\n");
    let err = app
        .commit(dir, "this should be blocked", vec!["SOLO"])
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
    app.commit(dir, "commit after returning from time travel", vec!["SOLO"])
        .expect("g c should succeed after g tt now");

    let log = app.log(dir);
    assert!(
        log.contains("commit after returning from time travel"),
        "commit made after time travel should be in the log\n{log}"
    );
}

#[test]
fn test_clean_commit_flow() {
    let app = TestApp::new();
    let repo1 = set_up_basic_repo(app.base_dir.path());
    let repo2 = clone_repo(app.base_dir.path(), "another_clone", "origin.git");

    write_file(repo1.as_path(), "hello.txt", "hello world\n");
    app.commit(repo1.as_path(), "add hello.txt", vec!["SOLO"])
        .expect("g c should succeed");

    let log = app.log(repo1.as_path());
    assert!(
        log.contains("add hello.txt"),
        "log should contain the commit message\n{log}"
    );

    app.pull(repo2.as_path()).expect("Pull should succeed");
    let log_b = app.log(repo2.as_path());
    assert!(
        log_b.contains("add hello.txt"),
        "commit should be visible from clone_b\n{log_b}"
    );
}

#[test]
fn test_fart_plays_fart_sound() {
    let app = TestApp::new();
    let path = app.base_dir.path().to_path_buf();
    app.fart(&path).expect("Fart should succeed");

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
