use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Result};

use crate::output::OutputSink;

pub fn base_cmd(dir: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.current_dir(dir)
        .env("GIT_EDITOR", "true")
        .env("GIT_TERMINAL_PROMPT", "0");
    cmd
}

pub fn git_passthrough(dir: &Path, args: &[&str], sink: &impl OutputSink) -> Result<()> {
    run_via_sink(dir, args, sink, Stdio::inherit())
}

pub fn git_passthrough_silent(dir: &Path, args: &[&str], sink: &impl OutputSink) -> Result<()> {
    run_via_sink(dir, args, sink, Stdio::null())
}

fn run_via_sink(dir: &Path, args: &[&str], sink: &impl OutputSink, stderr: Stdio) -> Result<()> {
    let mut cmd = base_cmd(dir);
    cmd.args(args).stderr(stderr);
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

pub fn is_rebasing(dir: &Path) -> bool {
    let gd = git_dir(dir);
    gd.join("rebase-merge").exists() || gd.join("rebase-apply").exists()
}

pub fn is_detached_head(dir: &Path) -> bool {
    let head_path = git_dir(dir).join("HEAD");
    std::fs::read_to_string(head_path)
        .map(|content| !content.trim_start().starts_with("ref:"))
        .unwrap_or(false)
}
