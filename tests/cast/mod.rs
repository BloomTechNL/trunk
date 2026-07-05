use crate::abilities::{
    AccessScenarioContext, ClockControl, Hear, ScenarioContext, UseFileSystem, UseGit, UseTrunk,
    VersionTrack,
};
use screenplay::Actor;

pub fn developer_bob(ctx: &ScenarioContext) -> Actor {
    let trunk = UseTrunk::new();
    let flag = trunk.app.fart_flag();
    let vers = trunk.app.update_flag();
    let clock = trunk.app.clock_flag();
    Actor::new("bob")
        .who_can(AccessScenarioContext::new(ctx))
        .who_can(trunk)
        .who_can(UseGit::new())
        .who_can(UseFileSystem)
        .who_can(Hear { played: flag })
        .who_can(VersionTrack { count: vers })
        .who_can(ClockControl { time: clock })
}

pub fn developer_kent(ctx: &ScenarioContext) -> Actor {
    let trunk = UseTrunk::new();
    let flag = trunk.app.fart_flag();
    let vers = trunk.app.update_flag();
    let clock = trunk.app.clock_flag();
    Actor::new("kent")
        .who_can(AccessScenarioContext::new(ctx))
        .who_can(trunk)
        .who_can(UseGit::new())
        .who_can(UseFileSystem)
        .who_can(Hear { played: flag })
        .who_can(VersionTrack { count: vers })
        .who_can(ClockControl { time: clock })
}
