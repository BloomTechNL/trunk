use std::path::Path;

use anyhow::Result;

use crate::git::git_passthrough;

// ---------------------------------------------------------------------------
// g r  — hard reset
// ---------------------------------------------------------------------------

pub fn cmd_reset(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["reset", "--hard"])?;
    git_passthrough(dir, &["clean", "-df", ":/"])
}
