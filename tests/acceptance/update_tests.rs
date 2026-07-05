use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::Update;
use crate::questions::TrunkVersion;
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
#[ignore = "TODO"]
fn test_application_updates_automatically_on_every_call() {}

#[test]
#[ignore = "TODO"]
fn test_application_only_updates_after_week_passes() {}

#[test]
#[ignore = "TODO"]
fn test_application_does_not_update_if_user_turns_this_off() {}

#[test]
#[ignore = "TODO"]
fn test_update_period_is_configurable() {}
