use crate::common::write_file::write_file;
use std::path::{Path, PathBuf};
use std::process::Command;

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

pub fn set_up_remote(dir: &Path) -> &str {
    git(dir, &["init", "--bare", "origin.git"]);
    "origin.git"
}

pub fn clone_repo(dir: &Path, repo_name: &str, from: &str) -> PathBuf {
    let repo_dir = dir.join(repo_name);
    git(dir, &["clone", from, repo_name]);
    git_config_identity(&repo_dir);
    repo_dir
}

pub fn initial_commit(repo_dir: &Path) {
    write_file(&repo_dir, "README.md", "# project\n");
    git(&repo_dir, &["add", "."]);
    git(&repo_dir, &["commit", "-m", "init"]);
    git(&repo_dir, &["push"]);
}

pub fn put_something_in_stash(repo_dir: &Path) {
    write_file(repo_dir, "stashed.txt", "stash me\n");
    git(repo_dir, &["add", "."]);
    git(repo_dir, &["stash"]);
}

pub fn commit_file(repo_dir: &Path) {
    write_file(repo_dir, "local.txt", "local only\n");
    git(repo_dir, &["add", "."]);
    git(repo_dir, &["commit", "-m", "local unpushed"]);
}

pub fn set_up_basic_repo(base_dir: &Path) -> PathBuf {
    let origin = set_up_remote(base_dir);
    let repo_dir = clone_repo(base_dir, "my_repo", origin);
    initial_commit(repo_dir.as_path());
    repo_dir
}
