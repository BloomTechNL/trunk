use std::path::Path;

use anyhow::{anyhow, bail, Result};

use crate::git::{git_capture, git_passthrough};
use crate::handler::Handler;
use crate::output::OutputSink;

// ---------------------------------------------------------------------------
// g tt  — time travel (detached HEAD)
// ---------------------------------------------------------------------------

fn default_branch(dir: &Path, sink: &impl OutputSink) -> String {
    if let Ok(out) = git_capture(dir, &["symbolic-ref", "refs/remotes/origin/HEAD"], sink) {
        if let Some(branch) = out.trim().strip_prefix("refs/remotes/origin/") {
            return branch.to_string();
        }
    }
    for candidate in &["main", "master"] {
        if git_capture(dir, &["rev-parse", "--verify", candidate], sink).is_ok() {
            return candidate.to_string();
        }
    }
    "main".to_string()
}

fn cmd_time_travel_now(dir: &Path, sink: &impl OutputSink) -> Result<()> {
    let branch = default_branch(dir, sink);
    git_passthrough(dir, &["checkout", &branch], sink)
}

pub struct TimeTravelHandler<'a, O: OutputSink> {
    sink: &'a O,
}

impl<'a, O: OutputSink> TimeTravelHandler<'a, O> {
    pub const fn new(sink: &'a O) -> Self {
        Self { sink }
    }
}

impl<O: OutputSink> Handler<(&Path, &str)> for TimeTravelHandler<'_, O> {
    fn handle(&self, (dir, target): (&Path, &str)) -> Result<()> {
        if target == "now" {
            return cmd_time_travel_now(dir, self.sink);
        }
        let hash = resolve_to_commit_hash(dir, target, self.sink)?;
        git_passthrough(dir, &["checkout", &hash], self.sink)
    }
}

fn resolve_to_commit_hash(dir: &Path, spec: &str, sink: &impl OutputSink) -> Result<String> {
    use git2::Repository;

    let repo = Repository::open(dir)?;

    if repo.find_branch(spec, git2::BranchType::Local).is_ok()
        || repo.find_branch(spec, git2::BranchType::Remote).is_ok()
    {
        bail!("'{spec}' is a branch name. g does not support branches.");
    }

    if let Some(hash) = try_resolve_time_ago(&repo, spec) {
        return Ok(hash);
    }

    let output = git_capture(dir, &["rev-parse", "--verify", spec], sink)?;
    let hash = output.trim().to_string();

    let obj = repo.revparse_single(&hash)?;
    let commit = obj
        .peel_to_commit()
        .map_err(|_| anyhow!("'{spec}' does not resolve to a commit"))?;
    Ok(commit.id().to_string())
}

fn try_resolve_time_ago(repo: &git2::Repository, spec: &str) -> Option<String> {
    let parts: Vec<&str> = spec.split_whitespace().collect();
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
