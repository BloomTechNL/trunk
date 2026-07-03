use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Result};

use crate::output::OutputSink;

#[must_use]
pub fn base_cmd(dir: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.current_dir(dir)
        .env("GIT_EDITOR", "true")
        .env("GIT_TERMINAL_PROMPT", "0");
    cmd
}

pub fn git_passthrough(dir: &Path, args: &[&str], sink: &impl OutputSink) -> Result<()> {
    let mut cmd = base_cmd(dir);
    cmd.args(args);
    let status = sink.run(&mut cmd)?;
    if status.success() {
        Ok(())
    } else {
        bail!(
            "git {} exited with status {}",
            args.join(" "),
            status.code().unwrap_or(-1)
        )
    }
}

pub fn git_capture(dir: &Path, args: &[&str], sink: &impl OutputSink) -> Result<String> {
    let mut cmd = base_cmd(dir);
    cmd.args(args);
    let (status, stdout) = sink.capture(&mut cmd)?;
    if status.success() {
        Ok(String::from_utf8_lossy(&stdout).into_owned())
    } else {
        bail!(
            "git {} exited with status {}",
            args.join(" "),
            status.code().unwrap_or(-1)
        )
    }
}

// ---------------------------------------------------------------------------
// Repository state helpers
// ---------------------------------------------------------------------------

pub fn git_dir(dir: &Path, sink: &impl OutputSink) -> PathBuf {
    if let Ok(out) = git_capture(dir, &["rev-parse", "--git-dir"], sink) {
        let trimmed = out.trim();
        let p = Path::new(trimmed);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            dir.join(p)
        }
    } else {
        dir.join(".git")
    }
}

pub fn is_rebasing(dir: &Path, sink: &impl OutputSink) -> bool {
    let gd = git_dir(dir, sink);
    gd.join("rebase-merge").exists() || gd.join("rebase-apply").exists()
}

/// Returns `true` when the working tree contains leftover conflict markers
/// (`<<<<<<<`). Uses `git2` to find files with unmerged index entries, then
/// checks whether their working-tree copies still contain conflict markers.
pub fn has_conflict_markers(dir: &Path, _sink: &impl OutputSink) -> bool {
    let Ok(repo) = git2::Repository::open(dir) else {
        return false;
    };
    let Ok(index) = repo.index() else {
        return false;
    };
    if !index.has_conflicts() {
        return false;
    }
    let Ok(conflicts) = index.conflicts() else {
        return false;
    };
    for conflict in conflicts {
        let Ok(c) = conflict else { continue };
        let entry = c.ancestor.as_ref().or(c.our.as_ref()).or(c.their.as_ref());
        let Some(path) = entry.and_then(|e| std::str::from_utf8(&e.path).ok()) else {
            continue;
        };
        let Ok(content) = std::fs::read(dir.join(path)) else {
            continue;
        };
        if content.windows(b"<<<<<<<".len()).any(|w| w == b"<<<<<<<") {
            return true;
        }
    }
    false
}

pub fn is_detached_head(dir: &Path, sink: &impl OutputSink) -> bool {
    let head_path = git_dir(dir, sink).join("HEAD");
    std::fs::read_to_string(head_path)
        .map(|content| !content.trim_start().starts_with("ref:"))
        .unwrap_or(false)
}
