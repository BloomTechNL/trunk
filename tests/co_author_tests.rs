use std::fs;
use tempfile::TempDir;
use anyhow::Result;

use crate::common::test_app::TestApp;
use crate::common::use_git::{set_up_basic_repo};

mod common;

#[test]
fn test_commit_solo() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    crate::common::write_file::write_file(repo_path, "solo.txt", "solo");
    app.commit(repo_path, "solo commit", Some("SOLO")).expect("SOLO commit should succeed");

    let log = g_cli::cmd_log(repo_path, true).expect("g l");
    assert!(log.contains("solo commit"));
    assert!(!log.contains("Co-authored-by:"));
}

#[test]
fn test_commit_missing_co_author_fails() {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    crate::common::write_file::write_file(repo_path, "fail.txt", "fail");
    let err = app.commit(repo_path, "missing co-author", None).expect_err("should fail");
    assert!(err.to_string().contains("co-author alias (@alias) or SOLO is required"));
}

#[test]
fn test_commit_with_alias() -> Result<()> {
    let app = TestApp::new();
    let repo = set_up_basic_repo(app.base_dir.path());
    let repo_path = repo.as_path();

    // Create a temporary home directory
    let temp_home = TempDir::new()?;
    let config_dir = temp_home.path().join(".config/trunk");
    fs::create_dir_all(&config_dir)?;
    let alias_file = config_dir.join("aliases");
    fs::write(alias_file, "jdoe:John Doe <jdoe@example.com>\n")?;

    // Use a subprocess or set env var if we were testing the binary, 
    // but here we are calling the function directly.
    // Since load_aliases reads from HOME, we set it.
    
    let old_home = std::env::var("HOME").ok();
    std::env::set_var("HOME", temp_home.path());

    crate::common::write_file::write_file(repo_path, "alias.txt", "alias");
    let result = app.commit(repo_path, "alias commit", Some("@jdoe"));
    
    // Restore HOME before assertions to be safe
    if let Some(val) = old_home {
        std::env::set_var("HOME", val);
    } else {
        std::env::remove_var("HOME");
    }

    result.expect("Commit with alias should succeed");

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

    let temp_home = TempDir::new()?;
    let old_home = std::env::var("HOME").ok();
    std::env::set_var("HOME", temp_home.path());

    crate::common::write_file::write_file(repo_path, "unknown.txt", "unknown");
    let result = app.commit(repo_path, "unknown alias", Some("@unknown"));

    if let Some(val) = old_home {
        std::env::set_var("HOME", val);
    } else {
        std::env::remove_var("HOME");
    }

    let err = result.expect_err("should fail with unknown alias");
    assert!(err.to_string().contains("Unknown co-author alias: @unknown"));
    assert!(err.to_string().contains("Please add it to ~/.config/trunk/aliases"));
    assert!(err.to_string().contains("alias:Name <email@example.com>"));

    Ok(())
}
