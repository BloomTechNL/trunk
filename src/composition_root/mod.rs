pub mod app_service;
pub mod dependencies;
pub mod slot;

use std::path::Path;

use crate::output::StdoutSink;
use crate::{
    RealClock, RealCoAuthorAliases, RealFartPlayer, RealLastUpdateStore, RealTrunkConfig,
    RealUpdater,
};

pub use app_service::AppService;
pub use dependencies::Dependencies;
pub use slot::Slot;

#[must_use]
pub fn assemble(
    config_dir: &Path,
) -> AppService<
    RealFartPlayer,
    RealCoAuthorAliases,
    RealUpdater<RealClock, RealLastUpdateStore>,
    StdoutSink,
    RealTrunkConfig,
> {
    let aliases_path = config_dir.join("aliases");
    let trunk_config_path = config_dir.join("trunk.json");
    let last_update_path = config_dir.join("last_update");

    let dependencies = Dependencies::new(
        Slot::register(|| RealFartPlayer),
        Slot::register(move || RealCoAuthorAliases::new(aliases_path)),
        Slot::register(move || {
            RealUpdater::new(RealClock, RealLastUpdateStore::new(last_update_path))
        }),
        Slot::register(|| StdoutSink),
        Slot::register(move || RealTrunkConfig::new(trunk_config_path)),
    );

    AppService::new(dependencies)
}
