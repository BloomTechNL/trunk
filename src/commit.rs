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

fn cmd_commit(dir: &Path, message: &str, co_author: Option<String>) -> Result<()> {
    if is_rebasing(dir) {
        bail!(
            "You are in the middle of resolving a conflict. Resolve the conflict and then run\n  g c --resolve"
        );
    }

    if is_detached_head(dir) {
        bail!("You are currently time travelling. Run `g tt now` to return to the present before making changes.");
    }

    let final_message = if let Some(author_input) = co_author {
        if author_input.to_uppercase() == "SOLO" {
            message.to_string()
        } else if author_input.starts_with('@') {
            let alias = &author_input[1..];
            let aliases = load_aliases()?;
            if let Some(full_author) = aliases.get(alias) {
                format!("{}\n\nCo-authored-by: {}", message, full_author)
            } else {
                bail!(
                    "Unknown co-author alias: @{}. Please add it to ~/.config/trunk/aliases in the format alias:Name <email@example.com>",
                    alias
                );
            }
        } else {
            bail!("Invalid co-author format. Use @alias or SOLO.");
        }
    } else {
        bail!("A co-author alias (@alias) or SOLO is required.");
    };

    git_passthrough(dir, &["add", "-A"])?;
    git_passthrough(dir, &["commit", "-m", &final_message])?;

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
    Message(String, Option<String>),
    Resolve,
    Abort,
}

pub struct CommitInput {
    pub repo: PathBuf,
    pub opt: CommitOpt,
}

fn load_aliases() -> Result<std::collections::HashMap<String, String>> {
    let mut aliases = std::collections::HashMap::new();

    let alias_file = if let Ok(path) = std::env::var("TRUNK_ALIASES_PATH") {
        PathBuf::from(path)
    } else {
        let home = std::env::var("HOME").map(PathBuf::from).or_else(|_| {
            std::env::var("USERPROFILE").map(PathBuf::from) // Windows support just in case
        });

        if let Ok(home_path) = home {
            home_path.join(".config/trunk/aliases")
        } else {
            return Ok(aliases);
        }
    };

    if alias_file.exists() {
        let content = std::fs::read_to_string(alias_file)?;
        for line in content.lines() {
            if let Some((alias, full)) = line.split_once(':') {
                aliases.insert(alias.trim().to_string(), full.trim().to_string());
            }
        }
    }
    Ok(aliases)
}

impl CommitInput {
    pub fn from_cli(
        repo: PathBuf,
        message: Option<String>,
        co_author: Option<String>,
        resolve: bool,
        abort: bool,
    ) -> Self {
        let opt: CommitOpt;
        if abort {
            opt = CommitOpt::Abort;
        } else if resolve {
            opt = CommitOpt::Resolve
        } else {
            opt = CommitOpt::Message(message.unwrap(), co_author)
        }
        CommitInput { repo, opt }
    }
}

pub fn commit(input: &CommitInput) -> Result<()> {
    match input.opt {
        CommitOpt::Message(ref message, ref co_author) => {
            cmd_commit(input.repo.as_path(), message, co_author.clone())
        }
        CommitOpt::Resolve => cmd_commit_resolve(input.repo.as_path()),
        CommitOpt::Abort => cmd_commit_abort(input.repo.as_path()),
    }
}
