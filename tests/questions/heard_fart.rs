use crate::abilities::Hear;
use screenplay::{Actor, Question};

pub struct HeardFart;

impl Question<bool> for HeardFart {
    fn answered_by(&self, actor: &Actor) -> bool {
        let hear = actor.ability::<Hear>().expect("actor needs Hear");
        hear.played.get()
    }
}
