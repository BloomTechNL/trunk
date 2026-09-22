use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{
    CloneRepo, Commit, Fart, InitialCommit, Mute, Pull, PutInStash, SetUpRemote, Unmute, WriteFile,
};
use crate::questions::{ConfigOutput, HeardFart, HeardHoldMusic, SoundTimeline};
use screenplay::*;

fn bob_in_a_clone(ctx: &ScenarioContext) -> Actor {
    let bob = developer_bob(ctx);
    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));
    bob
}

#[test]
fn muting_silences_the_hold_music() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = bob_in_a_clone(&ctx);

    bob.attempts_to((
        Mute,
        WriteFile {
            name: "hello.txt",
            content: "hello world\n",
        },
        Commit {
            message: Some("add hello.txt"),
            co_authors: vec!["SOLO"],
        },
        Ensure::that(HeardHoldMusic, is_false()),
        Ensure::that(SoundTimeline, does_not_contain("music start")),
    ));
}

#[test]
fn unmuting_restores_the_hold_music() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = bob_in_a_clone(&ctx);

    bob.attempts_to((
        Mute,
        Unmute,
        WriteFile {
            name: "hello.txt",
            content: "hello world\n",
        },
        Commit {
            message: Some("add hello.txt"),
            co_authors: vec!["SOLO"],
        },
        Ensure::that(SoundTimeline, contains("music start|git commit|music stop")),
    ));
}

#[test]
fn muting_does_not_silence_farts() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = bob_in_a_clone(&ctx);

    bob.attempts_to((Mute, Fart, Ensure::that(HeardFart, is_true())));
}

#[test]
fn muting_does_not_silence_the_stashed_work_fart() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = bob_in_a_clone(&ctx);

    bob.attempts_to((Mute, PutInStash, Pull, Ensure::that(HeardFart, is_true())));
}

#[test]
fn mute_is_visible_in_the_config() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = bob_in_a_clone(&ctx);

    bob.attempts_to((
        Ensure::that(ConfigOutput, contains("\"holdMusicMuted\": false")),
        Mute,
        Ensure::that(ConfigOutput, contains("\"holdMusicMuted\": true")),
        Unmute,
        Ensure::that(ConfigOutput, contains("\"holdMusicMuted\": false")),
    ));
}
