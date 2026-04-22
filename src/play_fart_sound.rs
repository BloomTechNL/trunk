use anyhow::{Context, Result};
use rust_embed::RustEmbed;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use std::io::Cursor;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

fn play_fart_sound() -> Result<()> {
    let random_index = rand::random_range(1..=6);
    let file_name = format!("fart-{}.mp3", random_index);

    let asset = Asset::get(&file_name)
        .with_context(|| format!("Failed to find {} in bundled assets", file_name))?;

    let mut handle = DeviceSinkBuilder::open_default_sink()
        .context("Could not open default audio output device")?;

    handle.log_on_drop(false);

    let player = Player::connect_new(&handle.mixer());

    let cursor = Cursor::new(asset.data);
    let source = Decoder::new(cursor)
        .context("Failed to decode MP3 data")?;

    player.append(source);
    player.sleep_until_end();

    Ok(())
}

pub trait FartPlayer {
    fn play(&self) -> Result<()>;
    fn play_asynchronously(&self) -> Result<()>;
}

pub struct RealFartPlayer;
impl FartPlayer for RealFartPlayer {
    fn play(&self) -> Result<()> {
        play_fart_sound()
    }

    fn play_asynchronously(&self) -> Result<()> {
        let exe = std::env::current_exe().context("Failed to get current executable path")?;

        std::process::Command::new(exe)
            .arg("internal-fart-daemon")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .context("Failed to spawn fart daemon process")?;

        Ok(())
    }
}
