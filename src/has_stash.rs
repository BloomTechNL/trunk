use std::path::Path;

#[must_use]
pub fn has_stash(dir: &Path) -> bool {
    git2::Repository::open(dir).is_ok_and(|repo| repo.find_reference("refs/stash").is_ok())
}
