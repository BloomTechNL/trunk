use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use screenplay::{Ability, Actor};
use tempfile::TempDir;

pub struct ActorContext {
    pub working_dir: PathBuf,
}

pub struct TestContext {
    pub base_dir: TempDir,
    pub actors: HashMap<&'static str, ActorContext>,
}

impl TestContext {
    pub fn new() -> Self {
        TestContext {
            base_dir: TempDir::new().expect("temp dir"),
            actors: HashMap::new(),
        }
    }
}

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

    pub fn actor_context(&self, actor: &Actor) -> Ref<ActorContext> {
        Ref::map(self.context.borrow(), |ctx| &ctx.actors[actor.name()])
    }

    pub fn actor_context_mut(&self, actor: &Actor) -> RefMut<ActorContext> {
        RefMut::map(self.context.borrow_mut(), |ctx| {
            ctx.actors.entry(actor.name()).or_insert(ActorContext {
                working_dir: PathBuf::new(),
            })
        })
    }
}
