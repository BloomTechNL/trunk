use std::path::Path;

use anyhow::Result;

use crate::git::{base_cmd, git_capture};

// ---------------------------------------------------------------------------
// g l / g s / g d  — read-only pass-throughs
// ---------------------------------------------------------------------------

pub fn cmd_log(dir: &Path, capture: bool) -> Result<String> {
    query_command(dir, &["log"], capture)
}

pub fn cmd_status(dir: &Path, capture: bool) -> Result<String> {
    query_command(dir, &["status"], capture)
}

pub fn cmd_diff(dir: &Path, capture: bool) -> Result<String> {
    query_command(dir, &["diff"], capture)
}

fn query_command(dir: &Path, args: &[&str], capture: bool) -> Result<String> {
    if capture {
        git_capture(dir, args)
    } else {
        // Ignore the exit status: pagers like `less` exit non-zero when the
        // user quits with 'q', which is normal and not an error.
        let _ = base_cmd(dir).args(args).status()?;
        Ok(String::new())
    }
}
