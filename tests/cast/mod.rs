use crate::abilities::{
    AccessScenarioContext, Hear, ScenarioContext, UseFileSystem, UseGit, UseTrunk,
};
use screenplay::Actor;

pub fn developer_bob(ctx: &ScenarioContext) -> Actor {
    let trunk = UseTrunk::new();
    let flag = trunk.app.fart_flag();
    Actor::new("bob")
        .who_can(AccessScenarioContext::new(ctx))
        .who_can(trunk)
        .who_can(UseGit::new())
        .who_can(UseFileSystem)
        .who_can(Hear { played: flag })
}

pub fn developer_kent(ctx: &ScenarioContext) -> Actor {
    let trunk = UseTrunk::new();
    let flag = trunk.app.fart_flag();
    Actor::new("kent")
        .who_can(AccessScenarioContext::new(ctx))
        .who_can(trunk)
        .who_can(UseGit::new())
        .who_can(UseFileSystem)
        .who_can(Hear { played: flag })
}
