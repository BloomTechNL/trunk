use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Result};

use crate::commit::{has_remote, has_remote_tracking};
use crate::git::{git_capture, git_passthrough, has_conflict_markers, is_detached_head};
use crate::handler::Handler;
use crate::output::OutputSink;

// ---------------------------------------------------------------------------
// g rv  — revert (interactive)
// ---------------------------------------------------------------------------

pub struct RevertInfo {
    pub short_hash: String,
    pub subject: String,
    pub author: String,
}

pub fn get_revert_info(dir: &Path, hash: &str, sink: &impl OutputSink) -> Result<RevertInfo> {
    use git2::Repository;
    let repo = Repository::open(dir)?;
    let obj = repo.revparse_single(hash)?;
    let commit = obj
        .peel_to_commit()
        .map_err(|_| anyhow!("'{hash}' does not point to a commit"))?;

    let short_hash = git_capture(dir, &["rev-parse", "--short", hash], sink)?
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

pub struct RevertHandler<'a, O: OutputSink> {
    sink: &'a O,
}

impl<'a, O: OutputSink> RevertHandler<'a, O> {
    pub const fn new(sink: &'a O) -> Self {
        Self { sink }
    }

    fn cmd_revert(&self, dir: &Path, hash: &str, bypass_prompt: bool) -> Result<()> {
        if is_detached_head(dir, self.sink) {
            bail!("You are currently time travelling. Run `g tt now` to return to the present before making changes.");
        }

        let info = get_revert_info(dir, hash, self.sink)?;

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

        let full_hash = git_capture(dir, &["rev-parse", hash], self.sink)?
            .trim()
            .to_string();
        git_passthrough(dir, &["revert", "--no-edit", &full_hash], self.sink)?;

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
                "\nAfter resolving the conflict, run\n  g rv --resolve\nOr run\n  g rv --abort\nTo give up (will permanently delete the revert commit)"
            );
            bail!("Conflict during rebase — see instructions above");
        }

        git_passthrough(dir, &["push"], self.sink)
    }

    fn cmd_revert_resolve(&self, dir: &Path) -> Result<()> {
        if has_conflict_markers(dir, self.sink) {
            bail!(
                "Your files contain unresolved conflict markers. Please resolve all conflicts before running `g rv --resolve`."
            );
        }
        git_passthrough(dir, &["add", "-A"], self.sink)?;
        git_passthrough(dir, &["rebase", "--continue"], self.sink)?;
        git_passthrough(dir, &["push"], self.sink)
    }

    fn cmd_revert_abort(&self, dir: &Path) -> Result<()> {
        git_passthrough(dir, &["rebase", "--abort"], self.sink)?;
        git_passthrough(dir, &["reset", "--hard", "HEAD~1"], self.sink)
    }
}

impl<O: OutputSink> Handler<&RevertInput> for RevertHandler<'_, O> {
    fn handle(&self, input: &RevertInput) -> Result<()> {
        match &input.opt {
            RevertOpt::Ref(reference) => {
                self.cmd_revert(&input.repo, reference, !input.interactive)
            }
            RevertOpt::Resolve => self.cmd_revert_resolve(&input.repo),
            RevertOpt::Abort => self.cmd_revert_abort(&input.repo),
        }
    }
}

pub enum RevertOpt {
    Ref(String),
    Resolve,
    Abort,
}

pub struct RevertInput {
    pub repo: PathBuf,
    pub opt: RevertOpt,
    pub interactive: bool,
}

impl RevertInput {
    pub fn from_cli(
        repo: PathBuf,
        hash: Option<String>,
        resolve: bool,
        abort: bool,
        interactive: bool,
    ) -> Self {
        let opt: RevertOpt;
        if abort {
            opt = RevertOpt::Abort;
        } else if resolve {
            opt = RevertOpt::Resolve;
        } else {
            opt = RevertOpt::Ref(hash.unwrap_or_else(|| "HEAD".to_string()));
        }
        Self {
            repo,
            opt,
            interactive,
        }
    }
}
