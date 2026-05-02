use anyhow::{Context, Result};
use git2::{Repository, ResetType, Status, StatusOptions};
use std::fs;
use std::path::Path;

pub fn cmd_reset(dir: &Path) -> Result<()> {
    let repo = Repository::discover(dir)
        .with_context(|| format!("Failed to discover git repository from {:?}", dir))?;

    let head = repo.head().context("Failed to resolve HEAD")?;
    let target = head
        .peel_to_commit()
        .context("Failed to peel HEAD to commit")?;

    repo.reset(target.as_object(), ResetType::Hard, None)
        .context("Failed to execute hard reset")?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(false)
        .include_ignored(false);

    let statuses = repo
        .statuses(Some(&mut opts))
        .context("Failed to read repository statuses")?;

    let workdir = repo.workdir().unwrap_or(dir);

    for entry in statuses.iter() {
        if entry.status().contains(Status::WT_NEW) {
            if let Some(path_str) = entry.path() {
                let full_path = workdir.join(path_str);

                if !full_path.exists() {
                    continue;
                }

                if full_path.is_dir() {
                    fs::remove_dir_all(&full_path)
                        .with_context(|| format!("Failed to remove directory {:?}", full_path))?;
                } else {
                    fs::remove_file(&full_path)
                        .with_context(|| format!("Failed to remove file {:?}", full_path))?;
                }
            }
        }
    }

    Ok(())
}
