use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{anyhow, bail, Result};

// ---------------------------------------------------------------------------
// Low-level git helpers
// ---------------------------------------------------------------------------

/// Apply env vars that prevent git from ever opening an interactive editor or
/// prompting for credentials.  Using `GIT_EDITOR=true` means git will run the
/// POSIX `true` utility (exits 0 immediately) instead of vim/nano, so
/// `rebase --continue` accepts the existing commit message without blocking.
fn base_cmd(dir: &Path) -> Command {
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
    let output = base_cmd(dir)
        .args(args)
        .stderr(Stdio::inherit())
        .output()?;
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
fn git_capture_silent(dir: &Path, args: &[&str]) -> Result<String> {
    let output = base_cmd(dir)
        .args(args)
        .stderr(Stdio::null())
        .output()?;
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
fn git_passthrough_silent(dir: &Path, args: &[&str]) -> Result<()> {
    let status = base_cmd(dir)
        .args(args)
        .stderr(Stdio::null())
        .status()?;
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
// Rebase-state detection
// ---------------------------------------------------------------------------

/// Returns `true` when the repository at `dir` is in the middle of a rebase
/// (either interactive or apply-based).
pub fn is_rebasing(dir: &Path) -> bool {
    // Locate the actual .git directory (works for both normal repos and
    // worktrees / clones where .git may be a file).
    let git_dir = git_dir(dir);
    git_dir.join("rebase-merge").exists() || git_dir.join("rebase-apply").exists()
}

fn git_dir(dir: &Path) -> std::path::PathBuf {
    // Ask git itself so we handle worktrees / submodules correctly.
    if let Ok(out) = git_capture(dir, &["rev-parse", "--git-dir"]) {
        let trimmed = out.trim();
        let p = std::path::Path::new(trimmed);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            dir.join(p)
        }
    } else {
        dir.join(".git")
    }
}

// ---------------------------------------------------------------------------
// g c  — commit + sync
// ---------------------------------------------------------------------------

/// Returns `true` when the current branch has a remote tracking branch
/// configured (i.e. it has been pushed at least once).
fn has_remote_tracking(dir: &Path) -> bool {
    git_capture(dir, &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"]).is_ok()
}

/// Returns `true` when at least one remote is configured for this repo.
fn has_remote(dir: &Path) -> bool {
    git_capture(dir, &["remote"])
        .map(|out| !out.trim().is_empty())
        .unwrap_or(false)
}

pub fn cmd_commit(dir: &Path, message: &str) -> Result<()> {
    if is_rebasing(dir) {
        bail!(
            "You are in the middle of resolving a conflict. Resolve the conflict and then run\n  g c --resolve"
        );
    }

    if is_detached_head(dir) {
        bail!("You are currently time travelling. Run `g tt now` to return to the present before making changes.");
    }

    git_passthrough(dir, &["add", "-A"])?;
    git_passthrough(dir, &["commit", "-m", message])?;

    if !has_remote(dir) {
        // Local-only repo — nothing to push to.
        return Ok(());
    }

    if !has_remote_tracking(dir) {
        // First push — no upstream exists yet, nothing to pull.
        return git_passthrough(dir, &["push", "--set-upstream", "origin", "HEAD"]);
    }

    let pull_result = git_passthrough(dir, &["pull", "--rebase"]);
    if pull_result.is_err() {
        eprintln!(
            "\nAfter resolving the conflict, run\n  g c --resolve\nOr run\n  g c --abort\nTo give up (will softly reset your commit)"
        );
        bail!("Conflict during rebase — see instructions above");
    }

    git_passthrough(dir, &["push"])
}

pub fn cmd_commit_resolve(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["add", "-A"])?;
    git_passthrough(dir, &["rebase", "--continue"])?;
    git_passthrough(dir, &["push"])
}

pub fn cmd_commit_abort(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["rebase", "--abort"])?;
    git_passthrough(dir, &["reset", "--soft", "HEAD~1"])
}

// ---------------------------------------------------------------------------
// g p  — pull (fast-forward only, guarded)
// ---------------------------------------------------------------------------

pub fn cmd_pull(dir: &Path) -> Result<()> {
    // Check for uncommitted changes.
    let porcelain = git_capture(dir, &["status", "--porcelain"])?;
    if !porcelain.trim().is_empty() {
        bail!(
            "You have uncommitted changes. Please commit them with `g c` before pulling."
        );
    }

    // Check for unpushed commits.
    let unpushed = git_capture(dir, &["log", "@{u}..HEAD", "--oneline"]).unwrap_or_default();
    if !unpushed.trim().is_empty() {
        bail!(
            "You have unpushed commits. Please push them with `g c` before pulling."
        );
    }

    git_passthrough(dir, &["pull", "--rebase"])
}

// ---------------------------------------------------------------------------
// g l / g s / g d  — read-only pass-throughs (captured for testability)
// ---------------------------------------------------------------------------

/// Run `git log` and return output.  When `capture` is false the output is
/// also printed to stdout so the terminal user sees it.
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

// ---------------------------------------------------------------------------
// g tt  — time travel (detached HEAD, no branches)
// ---------------------------------------------------------------------------

/// Returns `true` when HEAD is detached (i.e. not pointing at a branch ref).
/// We read `.git/HEAD` directly — it contains "ref: refs/heads/<branch>" when
/// attached, or a bare SHA when detached.
pub fn is_detached_head(dir: &Path) -> bool {
    let head_path = git_dir(dir).join("HEAD");
    std::fs::read_to_string(head_path)
        .map(|content| !content.trim_start().starts_with("ref:"))
        .unwrap_or(false)
}

/// Detect the repository's default branch name by inspecting the remote HEAD
/// symbolic ref, falling back to trying "main" then "master".
fn default_branch(dir: &Path) -> String {
    // Use the silent variant — this probe fails in repos where the remote HEAD
    // is not a symbolic ref, and the "fatal:" message would confuse the user.
    if let Ok(out) = git_capture_silent(dir, &["symbolic-ref", "refs/remotes/origin/HEAD"]) {
        if let Some(branch) = out.trim().strip_prefix("refs/remotes/origin/") {
            return branch.to_string();
        }
    }
    // Fall back: check which of main/master exists locally.
    for candidate in &["main", "master"] {
        if git_capture_silent(dir, &["rev-parse", "--verify", candidate]).is_ok() {
            return candidate.to_string();
        }
    }
    "main".to_string()
}

/// `g tt now` — return to the present by checking out the default branch.
pub fn cmd_time_travel_now(dir: &Path) -> Result<()> {
    let branch = default_branch(dir);
    // Use the silent variant: git prints "Switched to branch '...'" to stderr,
    // which is noise when `g tt now` is the UX boundary.
    git_passthrough_silent(dir, &["checkout", &branch])
}

pub fn cmd_time_travel(dir: &Path, target: &str) -> Result<()> {
    if target == "now" {
        return cmd_time_travel_now(dir);
    }
    // Resolve the target to a commit hash using git2 / git rev-parse so we
    // can guarantee we never check out a branch ref.
    let hash = resolve_to_commit_hash(dir, target)?;
    // Use the silent variant: git prints "Previous HEAD position was ..."
    // and "HEAD is now at ..." to stderr — noise that g supersedes.
    git_passthrough_silent(dir, &["checkout", &hash])
}

/// Resolve `spec` to a full commit SHA, refusing to resolve anything that
/// looks like (or resolves through) a branch ref.
fn resolve_to_commit_hash(dir: &Path, spec: &str) -> Result<String> {
    use git2::Repository;

    let repo = Repository::open(dir)?;

    // Reject if the spec names an existing local or remote branch.
    if repo.find_branch(spec, git2::BranchType::Local).is_ok()
        || repo.find_branch(spec, git2::BranchType::Remote).is_ok()
    {
        bail!("'{}' is a branch name. g does not support branches.", spec);
    }

    // Try to parse as a "time ago" string first.
    if let Some(hash) = try_resolve_time_ago(&repo, spec) {
        return Ok(hash);
    }

    // Fall back to git rev-parse (handles hashes, HEAD, etc.).
    let output = git_capture(dir, &["rev-parse", "--verify", spec])?;
    let hash = output.trim().to_string();

    // Double-check the resolved object is a commit, not a branch tip reached
    // via a symbolic ref.
    let obj = repo.revparse_single(&hash)?;
    let commit = obj
        .peel_to_commit()
        .map_err(|_| anyhow!("'{}' does not resolve to a commit", spec))?;
    Ok(commit.id().to_string())
}

/// Attempt to parse strings like "2 hours ago", "3 days ago", "1 week ago".
/// Returns `None` if the string doesn't match the pattern.
fn try_resolve_time_ago(repo: &git2::Repository, spec: &str) -> Option<String> {
    let parts: Vec<&str> = spec.split_whitespace().collect();
    // Expect "<number> <unit> ago"
    if parts.len() < 3 || parts.last() != Some(&"ago") {
        return None;
    }
    let n: i64 = parts[0].parse().ok()?;
    let unit = parts[1].to_lowercase();
    let seconds: i64 = match unit.trim_end_matches('s') {
        "second" => n,
        "minute" => n * 60,
        "hour" => n * 3600,
        "day" => n * 86_400,
        "week" => n * 604_800,
        _ => return None,
    };

    let target_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs() as i64
        - seconds;

    // Walk the HEAD history and find the newest commit <= target_time.
    let head = repo.head().ok()?.peel_to_commit().ok()?;
    let mut walk = repo.revwalk().ok()?;
    walk.push(head.id()).ok()?;
    walk.set_sorting(git2::Sort::TIME).ok()?;

    for oid in walk.flatten() {
        if let Ok(commit) = repo.find_commit(oid) {
            if commit.time().seconds() <= target_time {
                return Some(oid.to_string());
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// g r  — hard reset
// ---------------------------------------------------------------------------

pub fn cmd_reset(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["reset", "--hard"])
}

// ---------------------------------------------------------------------------
// g rv  — revert (interactive)
// ---------------------------------------------------------------------------

pub struct RevertInfo {
    pub short_hash: String,
    pub subject: String,
    pub author: String,
}

/// Fetch metadata about the commit to be reverted.
pub fn get_revert_info(dir: &Path, hash: &str) -> Result<RevertInfo> {
    use git2::Repository;
    let repo = Repository::open(dir)?;
    let obj = repo.revparse_single(hash)?;
    let commit = obj
        .peel_to_commit()
        .map_err(|_| anyhow!("'{}' does not point to a commit", hash))?;

    let short_hash = git_capture(dir, &["rev-parse", "--short", hash])?
        .trim()
        .to_string();
    let subject = commit
        .summary()
        .unwrap_or("<no message>")
        .to_string();
    let author = commit
        .author()
        .name()
        .unwrap_or("Unknown")
        .to_string();

    Ok(RevertInfo {
        short_hash,
        subject,
        author,
    })
}

/// Perform the revert.  When `bypass_prompt` is `true` the interactive
/// confirmation is skipped (used in tests).
pub fn cmd_revert(dir: &Path, hash: &str, bypass_prompt: bool) -> Result<()> {
    if is_detached_head(dir) {
        bail!("You are currently time travelling. Run `g tt now` to return to the present before making changes.");
    }

    let info = get_revert_info(dir, hash)?;

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

    // Resolve to full hash so git-revert always gets an unambiguous ref.
    let full_hash = git_capture(dir, &["rev-parse", hash])?.trim().to_string();

    git_passthrough(dir, &["revert", "--no-edit", &full_hash])?;

    if !has_remote(dir) {
        return Ok(());
    }

    if !has_remote_tracking(dir) {
        return git_passthrough(dir, &["push", "--set-upstream", "origin", "HEAD"]);
    }

    let pull_result = git_passthrough(dir, &["pull", "--rebase"]);
    if pull_result.is_err() {
        eprintln!(
            "\nAfter resolving the conflict, run\n  g rv --resolve\nOr run\n  g rv --abort\nTo give up (will permanently delete the revert commit)"
        );
        bail!("Conflict during rebase — see instructions above");
    }

    git_passthrough(dir, &["push"])
}

pub fn cmd_revert_resolve(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["add", "-A"])?;
    git_passthrough(dir, &["rebase", "--continue"])?;
    git_passthrough(dir, &["push"])
}

pub fn cmd_revert_abort(dir: &Path) -> Result<()> {
    git_passthrough(dir, &["rebase", "--abort"])?;
    // Hard-reset to destroy the revert commit entirely.
    git_passthrough(dir, &["reset", "--hard", "HEAD~1"])
}

