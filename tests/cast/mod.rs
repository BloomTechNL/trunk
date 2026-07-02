use crate::abilities::{AccessScenarioContext, ScenarioContext, UseFileSystem, UseGit, UseTrunk};
use screenplay::Actor;

pub fn developer_bob(ctx: &ScenarioContext) -> Actor {
    Actor::new()
        .who_can(AccessScenarioContext::new(ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem)
}

pub fn developer_kent(ctx: &ScenarioContext) -> Actor {
    Actor::new()
        .who_can(AccessScenarioContext::new(ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem)
}
