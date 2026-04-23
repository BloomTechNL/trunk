use std::path::Path;

pub fn has_stash(dir: &Path) -> bool {
    git2::Repository::open(dir)
            .map(|repo| repo.find_reference("refs/stash").is_ok())
            .unwrap_or(false)
}
