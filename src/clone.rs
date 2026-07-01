use std::path::Path;

use anyhow::Result;

use crate::git::git_passthrough;
use crate::output::OutputSink;

/// Clone a repository and configure basic git identity in the clone.
pub fn cmd_clone(
    dir: &Path,
    source: &str,
    destination: &str,
    sink: &impl OutputSink,
) -> Result<()> {
    git_passthrough(dir, &["clone", source, destination], sink)?;

    let dest_dir = dir.join(destination);

    git_passthrough(
        &dest_dir,
        &["config", "user.email", "test@example.com"],
        sink,
    )?;
    git_passthrough(&dest_dir, &["config", "user.name", "Test User"], sink)?;
    git_passthrough(&dest_dir, &["config", "commit.gpgsign", "false"], sink)?;
    git_passthrough(&dest_dir, &["config", "rebase.autostash", "false"], sink)?;

    Ok(())
}
