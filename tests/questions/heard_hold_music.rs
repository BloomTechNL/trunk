use crate::abilities::Hear;
use screenplay::{Ability, Actor, Question};

pub struct HeardHoldMusic;

impl Question<bool> for HeardHoldMusic {
    fn answered_by(&self, actor: &Actor) -> bool {
        Hear::by(actor).music_started.get()
    }
}
