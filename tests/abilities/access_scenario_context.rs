use std::cell::RefCell;
use std::rc::Rc;

use screenplay::Ability;
use tempfile::TempDir;

/// Shared state that every actor in the scenario can access.
///
/// Owns the temp directory where `origin.git` lives so both developers can
/// clone from the same remote.
pub struct TestContext {
    pub base_dir: TempDir,
}

impl TestContext {
    pub fn new() -> Self {
        TestContext {
            base_dir: TempDir::new().expect("temp dir"),
        }
    }
}

/// A reference-counted, interior-mutable handle to the shared [`TestContext`].
///
/// Created once by the test function. Each actor that needs shared state
/// receives an [`AccessScenarioContext`] ability cloned from this.
pub struct ScenarioContext {
    pub(crate) inner: Rc<RefCell<TestContext>>,
}

impl ScenarioContext {
    pub fn new(ctx: TestContext) -> Self {
        ScenarioContext {
            inner: Rc::new(RefCell::new(ctx)),
        }
    }
}

/// Ability that gives an [`Actor`] access to the shared [`ScenarioContext`].
///
/// The inner `Rc<RefCell<TestContext>>` is deliberately exposed — borrow it
/// with `.context.borrow()` or `.context.borrow_mut()` inside interactions
/// and questions.
pub struct AccessScenarioContext {
    pub context: Rc<RefCell<TestContext>>,
}

impl Ability for AccessScenarioContext {}

impl AccessScenarioContext {
    pub fn new(ctx: &ScenarioContext) -> Self {
        AccessScenarioContext {
            context: ctx.inner.clone(),
        }
    }
}
