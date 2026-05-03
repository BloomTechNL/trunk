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
    git_config_identity(&dir);
    "origin.git"
}

pub fn clone_repo(dir: &Path, repo_name: &str, from: &str) -> PathBuf {
    let repo_dir = dir.join(repo_name);
    git(dir, &["clone", from, repo_name]);
    git_config_identity(&repo_dir);
    repo_dir
}
