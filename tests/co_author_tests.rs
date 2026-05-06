use std::fs;
use std::sync::Mutex;
use tempfile::TempDir;
use anyhow::Result;

use crate::common::test_app::TestApp;
use crate::common::use_git::{set_up_basic_repo};
use crate::common::write_file::write_file;

mod common;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_commit_solo() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    write_file(repo_path, "solo.txt", "solo");
    app.commit(repo_path, "solo commit", Some("SOLO")).expect("SOLO commit should succeed");

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
    let err = app.commit(repo_path, "missing co-author", None).expect_err("should fail");
    assert!(err.to_string().contains("co-author alias (@alias) or SOLO is required"));
}

#[test]
fn test_commit_with_alias() -> Result<()> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    // Create a temporary home directory
    let temp_home = TempDir::new()?;
    let config_dir = temp_home.path().join(".config/trunk");
    fs::create_dir_all(&config_dir)?;
    let alias_file = config_dir.join("aliases");
    fs::write(&alias_file, "jdoe:John Doe <jdoe@example.com>\n")?;

    // Use TRUNK_ALIASES_PATH for robust testing in pipelines
    std::env::set_var("TRUNK_ALIASES_PATH", &alias_file);

    write_file(repo_path, "alias.txt", "alias");
    let result = app.commit(repo_path, "alias commit", Some("@jdoe"));
    
    // Clean up
    std::env::remove_var("TRUNK_ALIASES_PATH");

    result.expect("Commit with alias should succeed");

    let log = g_cli::cmd_log(repo_path, true).expect("g l");
    assert!(log.contains("alias commit"));
    assert!(log.contains("Co-authored-by: John Doe <jdoe@example.com>"));

    Ok(())
}

#[test]
fn test_commit_with_unknown_alias_fails() -> Result<()> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    let temp_home = TempDir::new()?;
    let alias_file = temp_home.path().join("aliases");
    fs::write(&alias_file, "known:Name <email@example.com>")?;

    std::env::set_var("TRUNK_ALIASES_PATH", &alias_file);

    write_file(repo_path, "unknown.txt", "unknown");
    let result = app.commit(repo_path, "unknown alias", Some("@unknown"));

    std::env::remove_var("TRUNK_ALIASES_PATH");

    let err = result.expect_err("should fail with unknown alias");
    assert!(err.to_string().contains("Unknown co-author alias: @unknown"));
    assert!(err.to_string().contains("Please add it to"));
    assert!(err.to_string().contains("alias:Name <email@example.com>"));

    Ok(())
}
