use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{CloneRepo, Fart, InitialCommit, Pull, PutInStash, SetUpRemote};
use crate::questions::HeardFart;
use screenplay::*;

#[test]
fn fart_plays_sound() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((Fart, Ensure::that(HeardFart, is_true())));
}

#[test]
fn pull_with_non_empty_stash_plays_fart() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((PutInStash, Pull, Ensure::that(HeardFart, is_true())));
}

#[test]
fn pull_with_empty_stash_does_not_play_fart() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((Pull, Ensure::that(HeardFart, is_false())));
}
