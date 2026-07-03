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
fn update_called_once_per_invocation() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((Update, Ensure::that(TrunkVersion, equals(1))));
    bob.attempts_to((Update, Ensure::that(TrunkVersion, equals(2))));
}
