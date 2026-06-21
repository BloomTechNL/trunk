use std::path::{Path, PathBuf};

use crate::config::TrunkConfig;
use crate::git::{git_capture, git_capture_silent, git_passthrough, is_detached_head, is_rebasing};
use crate::output::OutputSink;
use crate::CoAuthorAliases;
use anyhow::{bail, Result};

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

fn cmd_commit(
    dir: &Path,
    message: &str,
    co_authors: &dyn MessagePostfix,
    aliases: &impl CoAuthorAliases,
    config: &impl TrunkConfig,
    sink: &impl OutputSink,
) -> Result<()> {
    if is_rebasing(dir) {
        bail!(
            "You are in the middle of resolving a conflict. Resolve the conflict and then run\n  g c --resolve"
        );
    }

    if is_detached_head(dir) {
        bail!("You are currently time travelling. Run `g tt now` to return to the present before making changes.");
    }

    if !co_authors.is_explicit() && config.load().co_authors_required {
        bail!("You must either specify co-authors as @jane @john or specify that this is solo work with SOLO");
    }

    let postfix = co_authors.format_postfix(aliases)?;
    let final_message = format!("{}{}", message, postfix);

    git_passthrough(dir, &["add", "-A"], sink)?;
    git_passthrough(dir, &["commit", "-m", &final_message], sink)?;

    if !has_remote(dir) {
        return Ok(());
    }

    if !has_remote_tracking(dir) {
        return git_passthrough(dir, &["push", "--set-upstream", "origin", "HEAD"], sink);
    }

    let pull_result = git_passthrough(dir, &["pull", "--rebase"], sink);
    if pull_result.is_err() {
        eprintln!(
            "\nAfter resolving the conflict, run\n  g c --resolve\nOr run\n  g c --abort\nTo give up (will softly reset your commit)"
        );
        bail!("Conflict during rebase — see instructions above");
    }

    git_passthrough(dir, &["push"], sink)
}

fn cmd_commit_resolve(dir: &Path, sink: &impl OutputSink) -> Result<()> {
    git_passthrough(dir, &["add", "-A"], sink)?;
    git_passthrough(dir, &["rebase", "--continue"], sink)?;
    git_passthrough(dir, &["push"], sink)
}

fn cmd_commit_abort(dir: &Path, sink: &impl OutputSink) -> Result<()> {
    git_passthrough(dir, &["rebase", "--abort"], sink)?;
    git_passthrough(dir, &["reset", "--soft", "HEAD~1"], sink)
}

pub trait MessagePostfix {
    fn format_postfix(&self, aliases: &dyn CoAuthorAliases) -> Result<String>;

    fn is_explicit(&self) -> bool;
}

pub struct CoAuthors(Vec<String>);

pub struct ExplicitSolo;

pub struct ImplicitSolo;

impl MessagePostfix for CoAuthors {
    fn format_postfix(&self, aliases: &dyn CoAuthorAliases) -> Result<String> {
        let mut lines = Vec::new();
        for author_input in &self.0 {
            let alias = &author_input[1..];
            match aliases.format_alias(alias) {
                Some(full_author) => {
                    lines.push(format!("Co-authored-by: {}", full_author));
                }
                None => bail!(
                    "Unknown co-author alias: @{}. Please add it to ~/.config/trunk/aliases in the format alias:Name <email@example.com>\n",
                    alias,
                ),
            }
        }
        Ok(format!("\n\n{}", lines.join("\n")))
    }

    fn is_explicit(&self) -> bool {
        true
    }
}

impl MessagePostfix for ExplicitSolo {
    fn format_postfix(&self, _aliases: &dyn CoAuthorAliases) -> Result<String> {
        Ok("\n\n(Solo-work)".to_string())
    }

    fn is_explicit(&self) -> bool {
        true
    }
}

impl MessagePostfix for ImplicitSolo {
    fn format_postfix(&self, _aliases: &dyn CoAuthorAliases) -> Result<String> {
        Ok(String::new())
    }

    fn is_explicit(&self) -> bool {
        false
    }
}

pub enum CommitAction {
    Commit {
        message: String,
        co_authors: Box<dyn MessagePostfix>,
    },
    Resolve,
    Abort,
}

pub struct CommitInput {
    pub repo: PathBuf,
    pub action: CommitAction,
}

fn parse_co_authors(co_authors: Vec<String>) -> Result<Box<dyn MessagePostfix>> {
    let has_solo = co_authors.contains(&"SOLO".to_string());
    if has_solo && co_authors.len() == 1 {
        return Ok(Box::new(ExplicitSolo));
    }
    if has_solo {
        bail!("SOLO cannot be combined with other co-authors.");
    }
    if co_authors.len() > 0 {
        for author in &co_authors {
            if !author.starts_with('@') {
                bail!("Invalid co-author format. Use @alias or SOLO.");
            }
        }
        return Ok(Box::new(CoAuthors(co_authors)));
    }
    Ok(Box::new(ImplicitSolo))
}

impl CommitInput {
    pub fn from_cli(
        repo: PathBuf,
        message: Option<String>,
        co_authors: Vec<String>,
        resolve: bool,
        abort: bool,
    ) -> Result<Self> {
        let action = if abort {
            CommitAction::Abort
        } else if resolve {
            CommitAction::Resolve
        } else {
            CommitAction::Commit {
                message: message.unwrap(),
                co_authors: parse_co_authors(co_authors)?,
            }
        };
        Ok(CommitInput { repo, action })
    }
}

pub fn commit(
    input: &CommitInput,
    aliases: &impl CoAuthorAliases,
    config: &impl TrunkConfig,
    sink: &impl OutputSink,
) -> Result<()> {
    match &input.action {
        CommitAction::Commit {
            message,
            co_authors,
        } => cmd_commit(
            input.repo.as_path(),
            message,
            co_authors.as_ref(),
            aliases,
            config,
            sink,
        ),
        CommitAction::Resolve => cmd_commit_resolve(input.repo.as_path(), sink),
        CommitAction::Abort => cmd_commit_abort(input.repo.as_path(), sink),
    }
}
