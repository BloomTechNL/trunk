use anyhow::Result;

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
    app.commit(repo_path, "solo commit", Some("SOLO"))
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
    let err = app
        .commit(repo_path, "missing co-author", None)
        .expect_err("should fail");
    assert!(err
        .to_string()
        .contains("co-author alias (@alias) or SOLO is required"));
}

#[test]
fn test_commit_with_alias() -> Result<()> {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    app.set_aliases("jdoe:John Doe <jdoe@example.com>\n")?;

    write_file(repo_path, "alias.txt", "alias");
    app.commit(repo_path, "alias commit", Some("@jdoe"))?;

    let log = g_cli::cmd_log(repo_path, true).expect("g l");
    assert!(log.contains("alias commit"));
    assert!(log.contains("Co-authored-by: John Doe <jdoe@example.com>"));

    Ok(())
}

#[test]
fn test_commit_with_unknown_alias_fails() -> Result<()> {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    app.set_aliases("known:Name <email@example.com>")?;

    write_file(repo_path, "unknown.txt", "unknown");
    let result = app.commit(repo_path, "unknown alias", Some("@unknown"));

    let err = result.expect_err("should fail with unknown alias");
    assert!(err
        .to_string()
        .contains("Unknown co-author alias: @unknown"));
    assert!(err.to_string().contains("Please add it to"));
    assert!(err.to_string().contains("known:Name <email@example.com>"));

    Ok(())
}
