use crate::common::test_app::TestApp;
use crate::common::use_git::set_up_basic_repo;
use crate::common::write_file::write_file;

mod common;

#[test]
fn test_commit_solo() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    write_file(repo_path, "solo.txt", "solo");
    app.commit(repo_path, "solo commit", vec!["SOLO"])
        .expect("SOLO commit should succeed");

    let log = g_cli::cmd_log(repo_path, true).expect("g l");
    assert!(log.contains("solo commit"));
    assert!(log.contains("(Solo-work)"));
    assert!(!log.contains("Co-authored-by:"));
}

#[test]
fn test_commit_missing_co_author_fails() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    write_file(repo_path, "fail.txt", "fail");
    app.commit(repo_path, "no authors", vec![])
        .expect("Empty authors should be interpreted as SOLO");

    let log = g_cli::cmd_log(repo_path, true).expect("g l");
    assert!(log.contains("no authors"));
    assert!(log.contains("(Solo-work)"));
}

#[test]
fn test_commit_with_alias() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    app.add_alias("jdoe", "John Doe", "jdoe@example.com")
        .expect("should succeed");

    write_file(repo_path, "alias.txt", "alias");
    app.commit(repo_path, "alias commit", vec!["@jdoe"])
        .expect("should succeed");

    let log = g_cli::cmd_log(repo_path, true).expect("g l");
    assert!(log.contains("alias commit"));
    assert!(log.contains("Co-authored-by: John Doe <jdoe@example.com>"));
}

#[test]
fn test_commit_with_unknown_alias_fails() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    app.add_alias("known", "Name", "email@example.com")
        .expect("should succeed");

    write_file(repo_path, "unknown.txt", "unknown");
    let result = app.commit(repo_path, "unknown alias", vec!["@unknown"]);

    let err = result.expect_err("should fail with unknown alias");
    assert!(err
        .to_string()
        .contains("Unknown co-author alias: @unknown"));

    assert!(err.to_string().contains("Please add it to"));
    assert!(err.to_string().contains("known:Name <email@example.com>"));
}

#[test]
fn test_commit_multiple_authors() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    app.add_alias("jdoe", "John Doe", "jdoe@example.com")
        .expect("should succeed");
    app.add_alias("asmith", "Alice Smith", "asmith@example.com")
        .expect("should succeed");

    write_file(repo_path, "multi.txt", "multi");
    app.commit(repo_path, "multi commit", vec!["@jdoe", "@asmith"])
        .expect("should succeed");

    let log = g_cli::cmd_log(repo_path, true).expect("g l");
    assert!(log.contains("multi commit"));
    assert!(log.contains("Co-authored-by: John Doe <jdoe@example.com>"));
    assert!(log.contains("Co-authored-by: Alice Smith <asmith@example.com>"));
}

#[test]
fn test_commit_solo_with_others_fails() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    app.add_alias("jdoe", "John Doe", "jdoe@example.com")
        .expect("should succeed");

    write_file(repo_path, "invalid.txt", "invalid");
    let err = app
        .commit(repo_path, "invalid commit", vec!["SOLO", "@jdoe"])
        .expect_err("should fail");

    assert!(err
        .to_string()
        .contains("SOLO cannot be combined with other co-authors."));
}
