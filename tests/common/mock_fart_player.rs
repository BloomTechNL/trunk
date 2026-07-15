#![allow(dead_code)]
use g_cli::FartPlayer;
use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone)]
pub struct MockFartPlayer {
    played: Rc<Cell<bool>>,
}

impl MockFartPlayer {
    pub fn new() -> Self {
        Self {
            played: Rc::new(Cell::new(false)),
        }
    }

    pub fn inner(&self) -> Rc<Cell<bool>> {
        self.played.clone()
    }

    pub fn was_played(&self) -> bool {
        self.played.get()
    }
}

impl FartPlayer for MockFartPlayer {
    fn play(&self) -> anyhow::Result<()> {
        self.played.set(true);
        Ok(())
    }

    fn play_asynchronously(&self) -> anyhow::Result<()> {
        self.played.set(true);
        Ok(())
    }

    fn run_daemon(&self, dir: &Path) -> anyhow::Result<()> {
        g_cli::run_fart_daemon(self, dir)
    }
}
