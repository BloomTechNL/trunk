pub trait Clock {
    fn now_secs(&self) -> u64;
}

pub struct RealClock;

impl Clock for RealClock {
    fn now_secs(&self) -> u64 {
        #[allow(clippy::unwrap_used)]
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
