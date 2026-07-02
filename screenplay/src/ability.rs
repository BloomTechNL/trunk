//! Abilities — marker trait for capabilities an [`Actor`] can use.

/// Marker trait for abilities that can be given to an [`Actor`](super::Actor).
///
/// Abilities must be `'static` so they can be stored in a type-erased map
/// and retrieved later with [`Actor::ability`](super::Actor::ability).
///
/// # Example
///
/// ```rust
/// use screenplay::Ability;
///
/// struct WebBrowser { headless: bool }
/// impl Ability for WebBrowser {}
/// ```
use crate::Actor;

pub trait Ability: 'static {
    fn by(actor: &Actor) -> &Self
    where
        Self: Sized,
    {
        actor.ability::<Self>().expect("actor needs ability")
    }
}
