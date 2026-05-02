use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Result};

// ---------------------------------------------------------------------------
// Low-level git helpers
// ---------------------------------------------------------------------------

/// Apply env vars that prevent git from ever opening an interactive editor or
/// prompting for credentials.  Using `GIT_EDITOR=true` means git will run the
/// POSIX `true` utility (exits 0 immediately) instead of vim/nano, so
/// `rebase --continue` accepts the existing commit message without blocking.
pub fn base_cmd(dir: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.current_dir(dir)
        .env("GIT_EDITOR", "true")
        .env("GIT_TERMINAL_PROMPT", "0");
    cmd
}

/// Run a git command in `dir`, inheriting stdio so the user sees colorised
/// output.  Returns the process exit status as an `Err` if it is non-zero.
pub fn git_passthrough(dir: &Path, args: &[&str]) -> Result<()> {
    let status = base_cmd(dir).args(args).status()?;
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

/// Run a git command in `dir` and capture its stdout as a `String`.
pub fn git_capture(dir: &Path, args: &[&str]) -> Result<String> {
    let output = base_cmd(dir).args(args).stderr(Stdio::inherit()).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        bail!(
            "git {} exited with status {}",
            args.join(" "),
            output.status.code().unwrap_or(-1)
        )
    }
}

/// Like `git_capture` but discards stderr entirely.  Use for probe commands
/// where a failure is expected in some configurations and the error message
/// would be confusing to the user.
pub fn git_capture_silent(dir: &Path, args: &[&str]) -> Result<String> {
    let output = base_cmd(dir).args(args).stderr(Stdio::null()).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        bail!(
            "git {} exited with status {}",
            args.join(" "),
            output.status.code().unwrap_or(-1)
        )
    }
}

/// Like `git_passthrough` but discards stderr.  Use when git would otherwise
/// print informational noise that `g` supersedes with its own UX.
pub fn git_passthrough_silent(dir: &Path, args: &[&str]) -> Result<()> {
    let status = base_cmd(dir).args(args).stderr(Stdio::null()).status()?;
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

// ---------------------------------------------------------------------------
// Repository state helpers
// ---------------------------------------------------------------------------

pub fn git_dir(dir: &Path) -> PathBuf {
    if let Ok(out) = git_capture(dir, &["rev-parse", "--git-dir"]) {
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

/// Returns `true` when the repository at `dir` is in the middle of a rebase
/// (either interactive or apply-based).
pub fn is_rebasing(dir: &Path) -> bool {
    let gd = git_dir(dir);
    gd.join("rebase-merge").exists() || gd.join("rebase-apply").exists()
}

/// Returns `true` when HEAD is detached (i.e. not pointing at a branch ref).
pub fn is_detached_head(dir: &Path) -> bool {
    let head_path = git_dir(dir).join("HEAD");
    std::fs::read_to_string(head_path)
        .map(|content| !content.trim_start().starts_with("ref:"))
        .unwrap_or(false)
}
