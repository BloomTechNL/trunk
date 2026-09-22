use crate::abilities::{
    AccessScenarioContext, ClockControl, Hear, ScenarioContext, UseFileSystem, UseGit, UseTrunk,
    VersionTrack,
};
use screenplay::Actor;

pub fn developer_bob(ctx: &ScenarioContext) -> Actor {
    let trunk = UseTrunk::new();
    let flag = trunk.fart_flag();
    let vers = trunk.update_flag();
    let clock = trunk.clock_flag();
    let music_started = trunk.hold_music_started_flag();
    let music_playing = trunk.hold_music_playing_flag();
    let timeline = trunk.timeline();
    Actor::new("bob")
        .who_can(AccessScenarioContext::new(ctx))
        .who_can(trunk)
        .who_can(UseGit::new())
        .who_can(UseFileSystem)
        .who_can(Hear {
            played: flag,
            music_started,
            music_playing,
            timeline,
        })
        .who_can(VersionTrack { count: vers })
        .who_can(ClockControl { time: clock })
}

pub fn developer_kent(ctx: &ScenarioContext) -> Actor {
    let trunk = UseTrunk::new();
    let flag = trunk.fart_flag();
    let vers = trunk.update_flag();
    let clock = trunk.clock_flag();
    let music_started = trunk.hold_music_started_flag();
    let music_playing = trunk.hold_music_playing_flag();
    let timeline = trunk.timeline();
    Actor::new("kent")
        .who_can(AccessScenarioContext::new(ctx))
        .who_can(trunk)
        .who_can(UseGit::new())
        .who_can(UseFileSystem)
        .who_can(Hear {
            played: flag,
            music_started,
            music_playing,
            timeline,
        })
        .who_can(VersionTrack { count: vers })
        .who_can(ClockControl { time: clock })
}
