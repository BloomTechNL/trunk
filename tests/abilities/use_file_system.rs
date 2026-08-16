use std::fs;
use std::path::Path;

use screenplay::Ability;

pub struct UseFileSystem;

impl Ability for UseFileSystem {}

impl UseFileSystem {
    pub fn write_file(&self, dir: &Path, name: &str, content: &str) {
        fs::write(dir.join(name), content).expect("write file");
    }

    pub fn remove_file(&self, dir: &Path, name: &str) {
        fs::remove_file(dir.join(name)).expect("remove file");
    }

    pub fn create_dir(&self, dir: &Path, name: &str) {
        fs::create_dir(dir.join(name)).expect("create_dir");
    }

    #[allow(clippy::unused_self)]
    pub fn create_symlink(&self, dir: &Path, name: &str, target: &str) {
        std::os::unix::fs::symlink(target, dir.join(name)).expect("create symlink");
    }

    pub fn file_exists(&self, dir: &Path, name: &str) -> bool {
        dir.join(name).exists()
    }

    #[allow(clippy::unused_self)]
    pub fn path_exists(&self, dir: &Path, name: &str) -> bool {
        fs::symlink_metadata(dir.join(name)).is_ok()
    }

    pub fn read_file(&self, dir: &Path, name: &str) -> String {
        fs::read_to_string(dir.join(name)).expect("read file")
    }
}
