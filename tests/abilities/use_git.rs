use std::cell::RefCell;
use std::path::PathBuf;

use screenplay::Ability;

/// Ability to operate on a local git checkout.
///
/// Each actor's `UseGit` holds the path to their own repo clone. The repo
/// path is set during the arrange phase by [`CloneRepo`]; later interactions
/// like [`Commit`] or [`Pull`] read it automatically.
///
/// The shared base directory (where `origin.git` lives) is stored in
/// [`AccessScenarioContext`], not here.
pub struct UseGit {
    pub repo: RefCell<PathBuf>,
}

impl Ability for UseGit {}

impl UseGit {
    pub fn new() -> Self {
        UseGit {
            repo: RefCell::new(PathBuf::new()),
        }
    }
}
