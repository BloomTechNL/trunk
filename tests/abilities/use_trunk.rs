use crate::common::test_app::TestApp;
use screenplay::Ability;

pub struct UseTrunk {
    pub app: TestApp,
}

impl Ability for UseTrunk {}

impl UseTrunk {
    pub fn new() -> Self {
        UseTrunk {
            app: TestApp::new(),
        }
    }
}
