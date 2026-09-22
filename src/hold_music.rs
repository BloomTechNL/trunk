use anyhow::{Context, Result};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use std::io::Cursor;

use crate::assets::Asset;

const TRACK: &str = "stick-bug.mp3";

pub trait Playback {}

pub trait HoldMusic {
    fn start(&self) -> Result<Box<dyn Playback>>;
}

pub struct RealHoldMusic;

struct RealPlayback {
    player: Player,
    _handle: MixerDeviceSink,
}

impl Playback for RealPlayback {}

impl Drop for RealPlayback {
    fn drop(&mut self) {
        self.player.stop();
    }
}

impl HoldMusic for RealHoldMusic {
    fn start(&self) -> Result<Box<dyn Playback>> {
        let asset = Asset::get(TRACK)
            .with_context(|| format!("Failed to find {TRACK} in bundled assets"))?;

        let mut handle = DeviceSinkBuilder::open_default_sink()
            .context("Could not open default audio output device")?;

        handle.log_on_drop(false);

        let player = Player::connect_new(handle.mixer());

        let source = Decoder::new(Cursor::new(asset.data))
            .context("Failed to decode MP3 data")?
            .repeat_infinite();

        player.append(source);

        Ok(Box::new(RealPlayback {
            player,
            _handle: handle,
        }))
    }
}
