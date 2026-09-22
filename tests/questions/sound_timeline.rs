use crate::abilities::Hear;
use screenplay::{Ability, Actor, Question};

pub struct SoundTimeline;

impl Question<String> for SoundTimeline {
    fn answered_by(&self, actor: &Actor) -> String {
        Hear::by(actor).timeline.joined()
    }
}
