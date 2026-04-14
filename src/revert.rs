use std::path::Path;

use anyhow::{anyhow, bail, Result};

use crate::commit::{has_remote, has_remote_tracking};
use crate::git::{git_capture, git_passthrough, is_detached_head};

// ---------------------------------------------------------------------------
// g rv  — revert (interactive)
// ---------------------------------------------------------------------------

pub struct RevertInfo {
    pub short_hash: String,
    pub subject: String,
    pub author: String,
}

pub fn get_revert_info(dir: &Path, hash: &str) -> Result<RevertInfo> {
    use git2::Repository;
    let repo = Repository::open(dir)?;
    let obj = repo.revparse_single(hash)?;
    let commit = obj
        .peel_to_commit()
        .map_err(|_| anyhow!("'{}' does not point to a commit", hash))?;

    let short_hash = git_capture(dir, &["rev-parse", "--short", hash])?
        .trim()
        .to_string();
    let subject = commit.summary().unwrap_or("<no message>").to_string();
    let author = commit.author().name().unwrap_or("Unknown").to_string();

    Ok(RevertInfo {
        short_hash,
        subject,
        author,
    })
}

pub fn cmd_revert(dir: &Path, hash: &str, bypass_prompt: bool) -> Result<()> {
    if is_detached_head(dir) {
        bail!("You are currently time travelling. Run `g tt now` to return to the present before making changes.");
    }

    let info = get_revert_info(dir, hash)?;

    if !bypass_prompt {
        let prompt_text = format!(
            "⏪ Revert Commit: {} - \"{}\" (by {})\nAre you sure you want to revert this commit?",
            info.short_hash, info.subject, info.author
        );
        let confirmed = inquire::Confirm::new(&prompt_text)
            .with_default(false)
            .prompt()?;
        if !confirmed {
            println!("Aborted.");
            return Ok(());
        }
    }

    let full_hash = git_capture(dir, &["rev-parse", hash])?.trim().to_string();
    git_passthrough(dir, &["revert", "--no-edit", &full_hash])?;

    if !has_remote(dir) {
        return Ok(());
    }

    if !has_remote_tracking(dir) {
        return git_passthrough(dir, &["push", "--set-upstream", "origin", "HEAD"]);
    }

    let pull_result = git_passthrough(dir, &["pull", "--rebase"]);
    if pull_result.is_err() {
        eprintln!(
            "\nAfter resolving the conflict, run\n  g rv --resolve\nOr run\n  g rv --abort\nTo give up (will permanently delete the revert commit)"
        );
        bail!("Conflict during rebase — see instructions above");
    }

    git_passthrough(dir, &["push"])
}

pub fn cmd_revert_resolve(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["add", "-A"])?;
    git_passthrough(dir, &["rebase", "--continue"])?;
    git_passthrough(dir, &["push"])
}

pub fn cmd_revert_abort(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["rebase", "--abort"])?;
    git_passthrough(dir, &["reset", "--hard", "HEAD~1"])
}

