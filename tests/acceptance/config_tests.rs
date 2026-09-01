use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{CloneRepo, Config, InitialCommit, SetUpRemote};
use crate::questions::ConfigOutput;
use screenplay::*;

#[test]
fn config_with_no_flags_prints_the_current_global_config() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        Ensure::that(ConfigOutput, contains("\"coAuthorsRequired\": true")),
        Ensure::that(ConfigOutput, contains("\"autoUpdatePeriod\": 604800")),
    ));
}

#[test]
fn config_shows_local_override_for_co_authors_required_when_auto_update_period_is_still_global() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((Config {
        co_authors_required: Some(false),
        auto_update_period: None,
        local: true,
    },));

    bob.attempts_to((
        Ensure::that(ConfigOutput, contains("\"coAuthorsRequired\": false")),
        Ensure::that(ConfigOutput, contains("\"autoUpdatePeriod\": 604800")),
    ));
}

#[test]
fn config_shows_local_override_for_auto_update_period_when_co_authors_required_is_still_global() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((Config {
        co_authors_required: None,
        auto_update_period: Some(3_600),
        local: true,
    },));

    bob.attempts_to((
        Ensure::that(ConfigOutput, contains("\"coAuthorsRequired\": true")),
        Ensure::that(ConfigOutput, contains("\"autoUpdatePeriod\": 3600")),
    ));
}
