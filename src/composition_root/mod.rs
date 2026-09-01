pub mod app_service;
pub mod dependencies;
pub mod slot;

use std::path::Path;

use crate::output::StdoutSink;
use crate::{
    RealClock, RealCoAuthorAliases, RealFartPlayer, RealLastUpdateStore, RealTrunkConfig,
    RealUpdater, RepoAwareTrunkConfig,
};

pub use app_service::AppService;
pub use dependencies::Dependencies;
pub use slot::Slot;

#[must_use]
pub fn assemble(
    config_dir: &Path,
    repo_dir: &Path,
) -> AppService<
    RealFartPlayer,
    RealCoAuthorAliases,
    RealUpdater<RealClock, RealLastUpdateStore>,
    StdoutSink,
    RepoAwareTrunkConfig<RealTrunkConfig>,
> {
    let aliases_path = config_dir.join("aliases");
    let trunk_config_path = config_dir.join("trunk.json");
    let last_update_path = config_dir.join("last_update");
    let repo_dir = repo_dir.to_path_buf();

    let dependencies = Dependencies::new(
        Slot::register(|| RealFartPlayer),
        Slot::register(move || RealCoAuthorAliases::new(aliases_path)),
        Slot::register(move || {
            RealUpdater::new(RealClock, RealLastUpdateStore::new(last_update_path))
        }),
        Slot::register(|| StdoutSink),
        Slot::register(move || {
            RepoAwareTrunkConfig::new(RealTrunkConfig::new(trunk_config_path), repo_dir)
        }),
    );

    AppService::new(dependencies)
}
