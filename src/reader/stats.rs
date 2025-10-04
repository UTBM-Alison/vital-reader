use std::time::Instant;

pub struct SessionStats {
    total_bytes: u64,
    start_time: Instant,
}

impl SessionStats {
    pub fn new() -> Self {
        Self {
            total_bytes: 0,
            start_time: Instant::now(),
        }
    }

    pub fn add_bytes(&mut self, count: usize) {
        self.total_bytes += count as u64;
    }

    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    pub fn average_rate(&self) -> f64 {
        let elapsed_secs = self.elapsed().as_secs_f64();
        if elapsed_secs > 0.0 {
            self.total_bytes as f64 / elapsed_secs
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        self.total_bytes = 0;
        self.start_time = Instant::now();
    }
}

impl Default for SessionStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_tracking() {
        let mut stats = SessionStats::new();
        stats.add_bytes(100);
        stats.add_bytes(50);
        assert_eq!(stats.total_bytes(), 150);
    }
}