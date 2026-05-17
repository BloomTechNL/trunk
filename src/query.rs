use std::path::Path;

use anyhow::Result;

use crate::git::git_passthrough;
use crate::output::OutputSink;

// ---------------------------------------------------------------------------
// g l / g s / g d  — read-only pass-throughs
// ---------------------------------------------------------------------------

pub fn cmd_log(dir: &Path, sink: &impl OutputSink) -> Result<()> {
    git_passthrough(dir, &["log"], sink)
}

pub fn cmd_status(dir: &Path, sink: &impl OutputSink) -> Result<()> {
    git_passthrough(dir, &["status"], sink)
}

pub fn cmd_diff(dir: &Path, sink: &impl OutputSink) -> Result<()> {
    git_passthrough(dir, &["diff"], sink)
}
