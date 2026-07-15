use std::path::Path;

use anyhow::{bail, Result};

use crate::git::{git_capture, git_passthrough};
use crate::output::OutputSink;

// ---------------------------------------------------------------------------
// g p  — pull (fast-forward only, guarded)
// ---------------------------------------------------------------------------

pub struct PullHandler<'a, O: OutputSink> {
    sink: &'a O,
}

impl<'a, O: OutputSink> PullHandler<'a, O> {
    pub const fn new(sink: &'a O) -> Self {
        Self { sink }
    }

    pub fn handle(&self, dir: &Path) -> Result<()> {
        let porcelain = git_capture(dir, &["status", "--porcelain"], self.sink)?;
        if !porcelain.trim().is_empty() {
            bail!("You have uncommitted changes. Please commit them with `g c` before pulling.");
        }

        let unpushed =
            git_capture(dir, &["log", "@{u}..HEAD", "--oneline"], self.sink).unwrap_or_default();
        if !unpushed.trim().is_empty() {
            bail!("You have unpushed commits. Please push them with `g c` before pulling.");
        }

        git_passthrough(dir, &["pull", "--rebase"], self.sink)
    }
}
