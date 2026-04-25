use anyhow::{Context, Result};
use rust_embed::RustEmbed;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Cursor;
use std::time::Duration;
use rand::RngExt;

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
    fn run_daemon(&self, dir: &Path) -> Result<()>;
}

pub struct RealFartPlayer;
impl FartPlayer for RealFartPlayer {
    fn play(&self) -> Result<()> {
        play_fart_sound()
    }

    fn play_asynchronously(&self) -> Result<()> {
        let exe = std::env::current_exe().context("Failed to get current executable path")?;
        let dir = std::env::current_dir().context("Failed to get current directory")?;

        // 1. Play immediate fart
        std::process::Command::new(&exe)
            .arg("fart")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .context("Failed to spawn immediate fart process")?;

        // 2. Start/ensure background daemon
        if !is_daemon_running(&dir)? {
            std::process::Command::new(&exe)
                .arg("_fart_daemon")
                .current_dir(&dir)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .context("Failed to spawn fart daemon process")?;
        }

        Ok(())
    }

    fn run_daemon(&self, dir: &Path) -> Result<()> {
        run_fart_daemon(self, dir)
    }
}

pub fn run_fart_daemon(player: &dyn FartPlayer, dir: &Path) -> Result<()> {
    let pid = std::process::id();
    register_daemon(dir, pid)?;

    while crate::has_stash(dir) {
        let _ = player.play();
        // Use a shorter interval if we are in a test environment to speed up tests
        let is_test = std::env::var("DAEMON_TEST_FAST_MODE").is_ok();
        let sleep_secs = if is_test {
            rand::rng().random_range(1..2)
        } else {
            rand::rng().random_range(5..30)
        };
        std::thread::sleep(Duration::from_secs(sleep_secs));
    }

    unregister_daemon(dir)?;
    Ok(())
}

fn vault_path() -> PathBuf {
    PathBuf::from("/tmp/.trunk/fart_vault")
}

fn is_daemon_running(dir: &Path) -> Result<bool> {
    let vault = vault_path();
    if !vault.exists() {
        return Ok(false);
    }

    let abs_dir = fs::canonicalize(dir)?;
    let content = fs::read_to_string(&vault)?;
    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            let pid_str = parts[0];
            let path_str = parts[1];
            if path_str == abs_dir.to_string_lossy() {
                if let Ok(pid) = pid_str.parse::<i32>() {
                    // Check if process exists. On Unix, kill -0 pid checks existence.
                    let status = std::process::Command::new("kill")
                        .arg("-0")
                        .arg(pid.to_string())
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();
                    if let Ok(status) = status {
                        if status.success() {
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }
    Ok(false)
}

fn register_daemon(dir: &Path, pid: u32) -> Result<()> {
    let vault = vault_path();
    if let Some(parent) = vault.parent() {
        fs::create_dir_all(parent)?;
    }

    let abs_dir = fs::canonicalize(dir)?;
    let entry = format!("{}:{}\n", pid, abs_dir.to_string_lossy());

    let mut content = if vault.exists() {
        fs::read_to_string(&vault)?
    } else {
        String::new()
    };

    content.push_str(&entry);
    fs::write(vault, content)?;
    Ok(())
}

fn unregister_daemon(dir: &Path) -> Result<()> {
    let vault = vault_path();
    if !vault.exists() {
        return Ok(());
    }

    let abs_dir = fs::canonicalize(dir)?;
    let content = fs::read_to_string(&vault)?;
    let new_content: Vec<&str> = content
        .lines()
        .filter(|line| {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                parts[1] != abs_dir.to_string_lossy()
            } else {
                true
            }
        })
        .collect();

    fs::write(vault, new_content.join("\n") + if new_content.is_empty() { "" } else { "\n" })?;
    Ok(())
}
