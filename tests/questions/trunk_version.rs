use crate::abilities::VersionTrack;
use screenplay::{Ability, Actor, Question};

pub struct TrunkVersion;

impl Question<u32> for TrunkVersion {
    fn answered_by(&self, actor: &Actor) -> u32 {
        let track = VersionTrack::by(actor);
        track.count.get()
    }
}
