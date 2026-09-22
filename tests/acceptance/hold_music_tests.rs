use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{
    CloneRepo, Commit, InitialCommit, InstallPreCommitHook, SetUpRemote, WriteFile,
};
use crate::questions::{HeardHoldMusic, HoldMusicPlaying, SoundTimeline, Status};
use screenplay::*;

fn bob_in_a_clone(ctx: &ScenarioContext) -> Actor {
    let bob = developer_bob(ctx);
    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));
    bob
}

#[test]
fn hold_music_plays_across_the_commit() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = bob_in_a_clone(&ctx);

    bob.attempts_to((
        WriteFile {
            name: "hello.txt",
            content: "hello world\n",
        },
        Commit {
            message: Some("add hello.txt"),
            co_authors: vec!["SOLO"],
        },
        Ensure::that(SoundTimeline, contains("music start|git commit|music stop")),
        Ensure::that(HoldMusicPlaying, is_false()),
    ));
}

#[test]
fn hold_music_stops_when_the_pre_commit_hook_fails() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = bob_in_a_clone(&ctx);

    bob.attempts_to((
        InstallPreCommitHook { exit_code: 1 },
        WriteFile {
            name: "hello.txt",
            content: "hello world\n",
        },
        Ensure::that(
            doing(Commit {
                message: Some("add hello.txt"),
                co_authors: vec!["SOLO"],
            }),
            fails(),
        ),
        Ensure::that(HeardHoldMusic, is_true()),
        Ensure::that(HoldMusicPlaying, is_false()),
        Ensure::that(SoundTimeline, contains("music start|git commit|music stop")),
    ));
}

#[test]
fn hold_music_does_not_play_for_other_commands() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = bob_in_a_clone(&ctx);

    bob.attempts_to((
        Ensure::that(Status, contains("nothing to commit")),
        Ensure::that(HeardHoldMusic, is_false()),
    ));
}
