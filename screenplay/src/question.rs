//! Questions — retrieve information from the system through an [`Actor`].

use crate::actor::Actor;

/// A **Question** retrieves information from the system through the actor's
/// abilities.
///
/// The type parameter `T` is the answer type — it can be any type.
///
/// ```rust
/// use screenplay::*;
///
/// struct PageTitle;
/// impl Question<String> for PageTitle {
///     fn answered_by(&self, actor: &Actor) -> String {
///         // actor.ability::<Browsing>().unwrap().title()
///         "Home".to_string()
///     }
/// }
/// ```
pub trait Question<T> {
    fn answered_by(&self, actor: &Actor) -> T;
}
