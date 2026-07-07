use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{AdvanceTime, CloneRepo, Config, InitialCommit, SetUpRemote, Update};
use crate::questions::{Status, TrunkVersion};
use screenplay::*;

#[test]
fn update_increments_version() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((
        Ensure::that(TrunkVersion, equals(0)),
        Update,
        Ensure::that(TrunkVersion, equals(1)),
    ));
}

#[test]
fn test_application_updates_automatically_on_every_call() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        Ensure::that(TrunkVersion, equals(0)),
        Ensure::that(Status, contains("nothing to commit")),
        Ensure::that(TrunkVersion, equals(1)),
    ));
}

#[test]
fn test_application_only_updates_after_week_passes() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        Ensure::that(TrunkVersion, equals(0)),
        Ensure::that(Status, contains("nothing to commit")),
        Ensure::that(TrunkVersion, equals(1)),
    ));

    bob.attempts_to((AdvanceTime { secs: 604_800 },));

    bob.attempts_to((
        Ensure::that(Status, contains("nothing to commit")),
        Ensure::that(Status, contains("nothing to commit")),
        Ensure::that(TrunkVersion, equals(2)),
    ));
}

#[test]
fn test_application_does_not_update_if_user_turns_this_off() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));
    bob.attempts_to((Config {
        co_authors_required: None,
        auto_update_period: Some(0),
    },));

    bob.attempts_to((
        Ensure::that(TrunkVersion, equals(0)),
        Ensure::that(Status, contains("nothing to commit")),
        Ensure::that(TrunkVersion, equals(0)),
    ));
}

#[test]
fn user_does_not_see_auto_update_output() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        Ensure::that(TrunkVersion, equals(0)),
        Ensure::that(Status, does_not_contain("updating")),
        Ensure::that(TrunkVersion, equals(1)),
    ));
}

#[test]
fn explicit_update_works_even_when_auto_update_disabled() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));
    bob.attempts_to((Config {
        co_authors_required: None,
        auto_update_period: Some(0),
    },));

    bob.attempts_to((
        Ensure::that(TrunkVersion, equals(0)),
        Update,
        Ensure::that(TrunkVersion, equals(1)),
    ));
}

#[test]
fn test_update_period_is_configurable() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));
    bob.attempts_to((Config {
        co_authors_required: None,
        auto_update_period: Some(1),
    },));

    bob.attempts_to((
        Ensure::that(TrunkVersion, equals(0)),
        Ensure::that(Status, contains("nothing to commit")),
        Ensure::that(TrunkVersion, equals(1)),
    ));

    bob.attempts_to((AdvanceTime { secs: 1 },));

    bob.attempts_to((
        Ensure::that(Status, contains("nothing to commit")),
        Ensure::that(Status, contains("nothing to commit")),
        Ensure::that(TrunkVersion, equals(2)),
    ));
}
