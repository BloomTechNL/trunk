//! Interactions — actions an [`Actor`] can perform, plus tuple dispatch.

use crate::actor::Actor;

// ---------------------------------------------------------------------------
// Interaction — something the actor can *do*
// ---------------------------------------------------------------------------

/// An **Interaction** is an action that an [`Actor`] can perform.
///
/// Implement this trait for each user-level action (e.g. `NavigateTo`,
/// `FillIn`, `Click`). The actor's abilities are accessible through the
/// `actor` parameter.
///
/// ```rust
/// use screenplay::*;
///
/// struct Increment;
/// impl Interaction for Increment {
///     fn perform_as(&self, _actor: &Actor) {
///         // actor.ability::<Counter>().unwrap().inc();
///     }
/// }
/// ```
pub trait Interaction {
    fn perform_as(&self, actor: &Actor);
}

// ---------------------------------------------------------------------------
// Interactions — dispatch trait for tuples of interactions
// ---------------------------------------------------------------------------

/// Trait implemented for tuples of [`Interaction`]s (arity 1 through 10).
///
/// This is the internal dispatch mechanism behind
/// [`Actor::attempts_to`](crate::Actor::attempts_to). Library users do not
/// need to implement this trait directly.
pub trait Interactions {
    fn perform_all(&self, actor: &Actor);
}

/// Macro that generates an [`Interactions`] impl for a tuple of the given
/// arity. Each element is constrained to [`Interaction`] and its
/// [`perform_as`](Interaction::perform_as) is called in sequence.
macro_rules! impl_interactions {
    ($($idx:tt => $gen:ident),+) => {
        impl<$($gen: Interaction),+> Interactions for ($($gen,)+) {
            fn perform_all(&self, actor: &Actor) {
                $(self.$idx.perform_as(actor);)+
            }
        }
    };
}

// Generate impls for tuples of size 1 through 10.
impl_interactions!(0 => A0);
impl_interactions!(0 => A0, 1 => A1);
impl_interactions!(0 => A0, 1 => A1, 2 => A2);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5, 6 => A6);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5, 6 => A6, 7 => A7);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5, 6 => A6, 7 => A7, 8 => A8);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5, 6 => A6, 7 => A7, 8 => A8, 9 => A9);
