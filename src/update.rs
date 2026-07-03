use anyhow::Result;

pub trait Updater {
    /// Update the application to the latest version.
    ///
    /// # Errors
    ///
    /// Returns an error if the update command fails.
    fn update(&self) -> Result<()>;
}

pub struct RealUpdater;

impl Updater for RealUpdater {
    fn update(&self) -> Result<()> {
        std::process::Command::new("bash")
            .arg("-c")
            .arg("curl -fsSL https://raw.githubusercontent.com/BloomTechNL/trunk/main/scripts/install.sh | bash")
            .status()
            .map(|_| ())
            .map_err(anyhow::Error::from)
    }
}
