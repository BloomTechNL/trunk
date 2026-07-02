use crate::abilities::Hear;
use screenplay::{Ability, Actor, Question};

pub struct HeardFart;

impl Question<bool> for HeardFart {
    fn answered_by(&self, actor: &Actor) -> bool {
        let hear = Hear::by(actor);
        hear.played.get()
    }
}
