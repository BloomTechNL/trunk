use std::cell::RefCell;

use crate::common::test_app::TestApp;
use screenplay::Ability;

/// Ability to dispatch `g` subcommands (`g c`, `g p`, `g l`, `g s`, …).
pub struct UseTrunk {
    pub app: TestApp,
    /// Error message from the last `AttemptCommit`, if any.
    pub last_error: RefCell<Option<String>>,
}

impl Ability for UseTrunk {}

impl UseTrunk {
    pub fn new() -> Self {
        UseTrunk {
            app: TestApp::new(),
            last_error: RefCell::new(None),
        }
    }
}
