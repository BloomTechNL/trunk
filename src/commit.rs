use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

use crate::git::{git_capture, git_capture_silent, git_passthrough, is_detached_head, is_rebasing};

// ---------------------------------------------------------------------------
// Remote helpers (shared with revert)
// ---------------------------------------------------------------------------

/// Returns `true` when at least one remote is configured for this repo.
pub fn has_remote(dir: &Path) -> bool {
    git_capture_silent(dir, &["remote"])
        .map(|out| !out.trim().is_empty())
        .unwrap_or(false)
}

/// Returns `true` when the current branch has a remote tracking branch
/// configured (i.e. it has been pushed at least once).
pub fn has_remote_tracking(dir: &Path) -> bool {
    git_capture(
        dir,
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
    )
    .is_ok()
}

// ---------------------------------------------------------------------------
// g c  — commit + sync
// ---------------------------------------------------------------------------

fn cmd_commit(dir: &Path, message: &str) -> Result<()> {
    if is_rebasing(dir) {
        bail!(
            "You are in the middle of resolving a conflict. Resolve the conflict and then run\n  g c --resolve"
        );
    }

    if is_detached_head(dir) {
        bail!("You are currently time travelling. Run `g tt now` to return to the present before making changes.");
    }

    git_passthrough(dir, &["add", "-A"])?;
    git_passthrough(dir, &["commit", "-m", message])?;

    if !has_remote(dir) {
        return Ok(());
    }

    if !has_remote_tracking(dir) {
        return git_passthrough(dir, &["push", "--set-upstream", "origin", "HEAD"]);
    }

    let pull_result = git_passthrough(dir, &["pull", "--rebase"]);
    if pull_result.is_err() {
        eprintln!(
            "\nAfter resolving the conflict, run\n  g c --resolve\nOr run\n  g c --abort\nTo give up (will softly reset your commit)"
        );
        bail!("Conflict during rebase — see instructions above");
    }

    git_passthrough(dir, &["push"])
}

fn cmd_commit_resolve(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["add", "-A"])?;
    git_passthrough(dir, &["rebase", "--continue"])?;
    git_passthrough(dir, &["push"])
}

fn cmd_commit_abort(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["rebase", "--abort"])?;
    git_passthrough(dir, &["reset", "--soft", "HEAD~1"])
}

pub enum CommitOpt {
    Message(String),
    Resolve,
    Abort,
}

pub struct CommitInput {
    pub repo: PathBuf,
    pub opt: CommitOpt
}

impl CommitInput {
    pub fn from_cli(repo: PathBuf, message: Option<String>, resolve: bool, abort: bool) -> Self {
        let opt: CommitOpt;
        if abort {
            opt = CommitOpt::Abort;
        } else if resolve {
            opt = CommitOpt::Resolve
        } else {
            opt = CommitOpt::Message(message.unwrap())
        }
        CommitInput {
            repo,
            opt,
        }
    }
}


pub fn commit(input: &CommitInput) -> Result<()> {
    match input.opt {
        CommitOpt::Message(ref message) => cmd_commit(input.repo.as_path(), message),
        CommitOpt::Resolve => cmd_commit_resolve(input.repo.as_path()),
        CommitOpt::Abort => cmd_commit_abort(input.repo.as_path()),
    }
}
