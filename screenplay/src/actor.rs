//! The [`Actor`] — central abstraction that performs interactions.

use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::ability::Ability;
use crate::interaction::Interactions;

/// The **Actor** is the heart of the Screenplay Pattern.
///
/// An actor holds a set of type-erased [`Ability`] values and can
/// [`attempts_to`](Actor::attempts_to) perform one or more
/// [`Interaction`](crate::Interaction)s. Use [`who_can`](Actor::who_can) in a
/// builder style to equip the actor with abilities before the test scenario
/// begins.
///
/// # Example
///
/// ```rust
/// use screenplay::*;
///
/// struct Browsing;
/// impl Ability for Browsing {}
///
/// let user = Actor::new().who_can(Browsing);
/// assert!(user.ability::<Browsing>().is_some());
/// ```
pub struct Actor {
    abilities: HashMap<TypeId, Box<dyn Any>>,
}

impl Actor {
    /// Create a new actor with no abilities.
    ///
    /// Use [`who_can`](Actor::who_can) to add abilities in builder style.
    pub fn new() -> Self {
        Actor {
            abilities: HashMap::new(),
        }
    }

    /// Add an ability and return `Self` for chaining.
    ///
    /// If an ability of the same concrete type already exists it is silently
    /// replaced.
    ///
    /// ```rust
    /// use screenplay::*;
    ///
    /// struct Browsing;
    /// impl Ability for Browsing {}
    ///
    /// let actor = Actor::new()
    ///     .who_can(Browsing);
    /// ```
    pub fn who_can<A: Ability + 'static>(mut self, ability: A) -> Self {
        self.abilities.insert(TypeId::of::<A>(), Box::new(ability));
        self
    }

    /// Borrow an ability of type `A`, or `None` if the actor doesn't have it.
    ///
    /// ```rust
    /// use screenplay::*;
    ///
    /// struct Browsing;
    /// impl Ability for Browsing {}
    ///
    /// struct Logging;
    /// impl Ability for Logging {}
    ///
    /// let actor = Actor::new().who_can(Browsing);
    /// assert!(actor.ability::<Browsing>().is_some());
    /// assert!(actor.ability::<Logging>().is_none());
    /// ```
    pub fn ability<A: Ability + 'static>(&self) -> Option<&A> {
        self.abilities
            .get(&TypeId::of::<A>())
            .and_then(|boxed| boxed.downcast_ref::<A>())
    }

    /// Execute one or more [`Interaction`](crate::Interaction)s sequentially.
    ///
    /// Pass a **tuple** of interactions: `(a,)`, `(a, b)`, `(a, b, c)`, …
    /// (up to 10 elements). Each interaction's
    /// [`perform_as`](crate::Interaction::perform_as) is called in order.
    ///
    /// ```rust
    /// use screenplay::*;
    ///
    /// struct SayHi;
    /// impl Interaction for SayHi {
    ///     fn perform_as(&self, _actor: &Actor) { /* … */ }
    /// }
    ///
    /// let actor = Actor::new();
    /// actor.attempts_to((SayHi,));
    /// actor.attempts_to((SayHi, SayHi));
    /// ```
    pub fn attempts_to<T: Interactions>(&self, interactions: T) {
        interactions.perform_all(self);
    }
}

impl Default for Actor {
    fn default() -> Self {
        Self::new()
    }
}
