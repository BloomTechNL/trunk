use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use screenplay::Ability;

pub struct UseGit {
    pub repo: RefCell<PathBuf>,
}

impl Ability for UseGit {}

impl UseGit {
    pub fn new() -> Self {
        UseGit {
            repo: RefCell::new(PathBuf::new()),
        }
    }

    fn git(&self, args: &[&str]) {
        let dir = self.repo.borrow();
        let status = Command::new("git")
            .args(args)
            .current_dir(dir.as_path())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .env("GIT_EDITOR", "true")
            .env("GIT_TERMINAL_PROMPT", "0")
            .status()
            .expect("git command failed");
        assert!(status.success(), "git {} failed", args.join(" "));
    }

    fn git_at(dir: &Path, args: &[&str]) {
        let status = Command::new("git")
            .args(args)
            .current_dir(dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .env("GIT_EDITOR", "true")
            .env("GIT_TERMINAL_PROMPT", "0")
            .status()
            .expect("git command failed");
        assert!(status.success(), "git {} failed", args.join(" "));
    }

    #[allow(clippy::unused_self)]
    pub fn set_up_remote(&self, dir: &Path) {
        Self::git_at(dir, &["init", "--bare", "origin.git"]);
    }

    pub fn clone_repo(&self, dir: &Path, repo_name: &str, from: &str) -> PathBuf {
        let repo_dir = dir.join(repo_name);
        Self::git_at(dir, &["clone", from, repo_name]);
        Self::configure_identity(&repo_dir);
        self.repo.borrow_mut().clone_from(&repo_dir);
        repo_dir
    }

    fn configure_identity(dir: &Path) {
        for (k, v) in &[
            ("user.email", "test@example.com"),
            ("user.name", "Test User"),
            ("commit.gpgsign", "false"),
            ("rebase.autostash", "false"),
        ] {
            Command::new("git")
                .args(["config", k, v])
                .current_dir(dir)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .expect("git config");
        }
    }

    pub fn initial_commit(&self) {
        let dir = self.repo.borrow();
        fs::write(dir.join("README.md"), "# project\n").expect("write README");
        drop(dir);
        self.git(&["add", "."]);
        self.git(&["commit", "-m", "init"]);
        self.git(&["push"]);
    }

    pub fn put_something_in_stash(&self) {
        let dir = self.repo.borrow();
        fs::write(dir.join("stashed.txt"), "stash me\n").expect("write file");
        drop(dir);
        self.git(&["add", "."]);
        self.git(&["stash"]);
    }

    pub fn commit_file(&self) {
        let dir = self.repo.borrow();
        fs::write(dir.join("local.txt"), "local only\n").expect("write file");
        drop(dir);
        self.git(&["add", "."]);
        self.git(&["commit", "-m", "local unpushed"]);
    }
}
