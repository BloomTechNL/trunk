use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::ability::Ability;
use crate::interaction::Interactions;

pub struct Actor {
    name: &'static str,
    abilities: HashMap<TypeId, Box<dyn Any>>,
}

impl Actor {
    pub fn new(name: &'static str) -> Self {
        Actor {
            name,
            abilities: HashMap::new(),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn who_can<A: Ability + 'static>(mut self, ability: A) -> Self {
        self.abilities.insert(TypeId::of::<A>(), Box::new(ability));
        self
    }

    pub fn ability<A: Ability + 'static>(&self) -> Option<&A> {
        self.abilities
            .get(&TypeId::of::<A>())
            .and_then(|boxed| boxed.downcast_ref::<A>())
    }

    pub fn attempts_to<T: Interactions>(&self, interactions: T) {
        interactions.perform_all(self);
    }
}
