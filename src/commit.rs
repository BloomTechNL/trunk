use std::path::{Path, PathBuf};

use crate::config::TrunkConfig;
use crate::git::{
    git_capture, git_passthrough, has_conflict_markers, is_detached_head, is_rebasing,
};
use crate::handler::Handler;
use crate::output::OutputSink;
use crate::CoAuthorAliases;
use anyhow::{bail, Result};

// ---------------------------------------------------------------------------
// Remote helpers (shared with revert)
// ---------------------------------------------------------------------------

/// Returns `true` when at least one remote is configured for this repo.
pub fn has_remote(dir: &Path, sink: &impl OutputSink) -> bool {
    git_capture(dir, &["remote"], sink).is_ok_and(|out| !out.trim().is_empty())
}

/// Returns `true` when the current branch has a remote tracking branch
/// configured (i.e. it has been pushed at least once).
pub fn has_remote_tracking(dir: &Path, sink: &impl OutputSink) -> bool {
    git_capture(
        dir,
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
        sink,
    )
    .is_ok()
}

// ---------------------------------------------------------------------------
// g c  — commit + sync
// ---------------------------------------------------------------------------

pub struct CommitHandler<'a, CA: CoAuthorAliases, TC: TrunkConfig, O: OutputSink> {
    aliases: &'a CA,
    config: &'a TC,
    sink: &'a O,
}

impl<'a, CA: CoAuthorAliases, TC: TrunkConfig, O: OutputSink> CommitHandler<'a, CA, TC, O> {
    pub const fn new(aliases: &'a CA, config: &'a TC, sink: &'a O) -> Self {
        Self {
            aliases,
            config,
            sink,
        }
    }

    fn cmd_commit(&self, dir: &Path, message: &str, co_authors: &dyn MessagePostfix) -> Result<()> {
        if is_rebasing(dir, self.sink) {
            bail!(
                "You are in the middle of resolving a conflict. Resolve the conflict and then run\n  g c --resolve"
            );
        }

        if is_detached_head(dir, self.sink) {
            bail!("You are currently time travelling. Run `g tt now` to return to the present before making changes.");
        }

        if !co_authors.is_explicit() && self.config.load().co_authors_required {
            bail!("You must either specify co-authors as @jane @john or specify that this is solo work with SOLO");
        }

        let postfix = co_authors.format_postfix(self.aliases)?;
        let final_message = format!("{message}{postfix}");

        git_passthrough(dir, &["add", "-A"], self.sink)?;
        git_passthrough(dir, &["commit", "-m", &final_message], self.sink)?;

        if !has_remote(dir, self.sink) {
            return Ok(());
        }

        if !has_remote_tracking(dir, self.sink) {
            return git_passthrough(
                dir,
                &["push", "--set-upstream", "origin", "HEAD"],
                self.sink,
            );
        }

        let pull_result = git_passthrough(dir, &["pull", "--rebase"], self.sink);
        if pull_result.is_err() {
            eprintln!(
                "\nAfter resolving the conflict, run\n  g c --resolve\nOr run\n  g c --abort\nTo give up (will softly reset your commit)"
            );
            bail!("Conflict during rebase — see instructions above");
        }

        git_passthrough(dir, &["push"], self.sink)
    }

    fn cmd_commit_resolve(&self, dir: &Path) -> Result<()> {
        if has_conflict_markers(dir, self.sink) {
            bail!(
                "Your files contain unresolved conflict markers. Please resolve all conflicts before running `g c --resolve`."
            );
        }
        git_passthrough(dir, &["add", "-A"], self.sink)?;
        git_passthrough(dir, &["rebase", "--continue"], self.sink)?;
        git_passthrough(dir, &["push"], self.sink)
    }

    fn cmd_commit_abort(&self, dir: &Path) -> Result<()> {
        git_passthrough(dir, &["rebase", "--abort"], self.sink)?;
        git_passthrough(dir, &["reset", "--soft", "HEAD~1"], self.sink)
    }
}

impl<CA: CoAuthorAliases, TC: TrunkConfig, O: OutputSink> Handler<&CommitInput>
    for CommitHandler<'_, CA, TC, O>
{
    fn handle(&self, input: &CommitInput) -> Result<()> {
        match &input.action {
            CommitAction::Commit {
                message,
                co_authors,
            } => self.cmd_commit(input.repo.as_path(), message, co_authors.as_ref()),
            CommitAction::Resolve => self.cmd_commit_resolve(input.repo.as_path()),
            CommitAction::Abort => self.cmd_commit_abort(input.repo.as_path()),
        }
    }
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
                    lines.push(format!("Co-authored-by: {full_author}"));
                }
                None => bail!(
                    "Unknown co-author alias: @{alias}. Please add it to ~/.config/trunk/aliases in the format alias:Name <email@example.com>\n",
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
    if !co_authors.is_empty() {
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
        Ok(Self { repo, action })
    }
}
