use std::path::Path;

use anyhow::{bail, Result};

use crate::git::{git_capture, git_passthrough};

// ---------------------------------------------------------------------------
// g p  — pull (fast-forward only, guarded)
// ---------------------------------------------------------------------------

pub fn cmd_pull(dir: &Path) -> Result<()> {
    let porcelain = git_capture(dir, &["status", "--porcelain"])?;
    if !porcelain.trim().is_empty() {
        bail!("You have uncommitted changes. Please commit them with `g c` before pulling.");
    }

    let unpushed = git_capture(dir, &["log", "@{u}..HEAD", "--oneline"]).unwrap_or_default();
    if !unpushed.trim().is_empty() {
        bail!("You have unpushed commits. Please push them with `g c` before pulling.");
    }

    git_passthrough(dir, &["pull", "--rebase"])
}

