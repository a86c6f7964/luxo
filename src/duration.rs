use std::time::Duration;

pub trait Millis {
    fn as_millis(&self) -> u64;
}

impl Millis for Duration {
    fn as_millis(&self) -> u64 {
        return (self.as_secs() * 1_000) + (self.subsec_nanos() / 1_000_000) as u64;
    }
}