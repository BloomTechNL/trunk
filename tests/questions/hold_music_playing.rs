use crate::abilities::Hear;
use screenplay::{Ability, Actor, Question};

pub struct HoldMusicPlaying;

impl Question<bool> for HoldMusicPlaying {
    fn answered_by(&self, actor: &Actor) -> bool {
        Hear::by(actor).music_playing.get()
    }
}
