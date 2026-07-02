use common::test_app::TestApp;
use common::use_git::{put_something_in_stash, set_up_basic_repo};

mod common;

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
