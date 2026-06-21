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

    let log = app.log(repo_path);
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
    let err = app
        .commit(repo_path, "no authors", vec![])
        .expect_err("No empty co-author list allowed");

    assert!(err
        .to_string()
        .contains( "You must either specify co-authors as @jane @john or specify that this is solo work with SOLO"
        ));
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

    let log = app.log(repo_path);
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

    let log = app.log(repo_path);

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
        .commit(repo_path, "invalid commit", vec!["@jdoe", "SOLO"])
        .expect_err("should fail");

    assert!(err
        .to_string()
        .contains("SOLO cannot be combined with other co-authors."));
}

#[test]
fn test_commit_succeeds_without_co_authors_when_config_disabled() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    app.config(repo_path, Some(false))
        .expect("config should succeed");

    write_file(repo_path, "noauthor.txt", "content");
    app.commit(repo_path, "commit without co-authors", vec![])
        .expect("should succeed when coAuthorsRequired is false");

    let log = app.log(repo_path);
    assert!(log.contains("commit without co-authors"));
    assert!(!log.contains("(Solo-work)"));
}

#[test]
fn test_commit_with_co_authors_when_config_disabled() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    app.add_alias("jdoe", "John Doe", "jdoe@example.com")
        .expect("should succeed");

    app.config(repo_path, Some(false))
        .expect("config should succeed");

    write_file(repo_path, "coauthor.txt", "content");
    app.commit(repo_path, "commit with co-author", vec!["@jdoe"])
        .expect("should succeed when coAuthorsRequired is false");

    let log = app.log(repo_path);
    assert!(log.contains("Co-authored-by: John Doe <jdoe@example.com>"));
}

#[test]
fn test_commit_solo_when_config_disabled() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    app.config(repo_path, Some(false))
        .expect("config should succeed");

    write_file(repo_path, "solo.txt", "solo");
    app.commit(repo_path, "solo commit", vec!["SOLO"])
        .expect("SOLO should still work when coAuthorsRequired is false");

    let log = app.log(repo_path);
    assert!(log.contains("solo commit"));
    assert!(log.contains("(Solo-work)"));
}
