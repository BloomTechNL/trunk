use anyhow::Result;

use crate::config::TrunkConfig;
use crate::{Clock, LastUpdateStore};

pub trait Updater {
    /// Update the application to the latest version.
    ///
    /// # Errors
    ///
    /// Returns an error if the update command fails.
    fn update(&self) -> Result<()>;

    fn auto_update(&self, config: &impl TrunkConfig) -> Result<()> {
        let cfg = config.load();
        if cfg.auto_update_period == 0 {
            return Ok(());
        }
        let now = self.clock().now_secs();
        let should_update = self
            .last_update_store()
            .read()
            .is_none_or(|prev| now - prev >= cfg.auto_update_period);
        if should_update {
            self.last_update_store().write(now)?;
            self.update()?;
        }
        Ok(())
    }

    fn clock(&self) -> &dyn Clock;

    fn last_update_store(&self) -> &dyn LastUpdateStore;
}

pub struct RealUpdater<C: Clock, LS: LastUpdateStore> {
    clock: C,
    last_update_store: LS,
}

impl<C: Clock, LS: LastUpdateStore> RealUpdater<C, LS> {
    pub const fn new(clock: C, last_update_store: LS) -> Self {
        Self {
            clock,
            last_update_store,
        }
    }
}

impl<C: Clock, LS: LastUpdateStore> Updater for RealUpdater<C, LS> {
    fn update(&self) -> Result<()> {
        std::process::Command::new("bash")
            .arg("-c")
            .arg("curl -fsSL https://raw.githubusercontent.com/BloomTechNL/trunk/main/scripts/install.sh | bash")
            .status()
            .map(|_| ())
            .map_err(anyhow::Error::from)
    }

    fn clock(&self) -> &dyn Clock {
        &self.clock
    }

    fn last_update_store(&self) -> &dyn LastUpdateStore {
        &self.last_update_store
    }
}
