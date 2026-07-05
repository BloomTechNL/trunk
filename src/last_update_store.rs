use anyhow::Result;
use std::path::PathBuf;

pub trait LastUpdateStore {
    fn read(&self) -> Option<u64>;

    fn write(&self, timestamp: u64) -> Result<()>;
}

pub struct RealLastUpdateStore {
    path: PathBuf,
}

impl RealLastUpdateStore {
    #[must_use]
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl LastUpdateStore for RealLastUpdateStore {
    fn read(&self) -> Option<u64> {
        let content = std::fs::read_to_string(&self.path).ok()?;
        content.trim().parse().ok()
    }

    fn write(&self, timestamp: u64) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, timestamp.to_string())?;
        Ok(())
    }
}
