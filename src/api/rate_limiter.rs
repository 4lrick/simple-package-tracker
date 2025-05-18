use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

#[derive(Clone)]
pub struct RateLimiter {
    last_request: Arc<AtomicU64>,
    min_interval: Duration,
}

impl RateLimiter {
    pub fn new(requests_per_second: u64) -> Self {
        Self {
            last_request: Arc::new(AtomicU64::new(0)),
            min_interval: Duration::from_millis(1000 / requests_per_second),
        }
    }

    pub async fn wait_for_next_request(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_millis() as u64;
        let last = self.last_request.load(Ordering::Relaxed);
        let elapsed = now.saturating_sub(last);
        
        if elapsed < self.min_interval.as_millis() as u64 {
            let wait_time = self.min_interval.as_millis() as u64 - elapsed;
            tokio::time::sleep(Duration::from_millis(wait_time)).await;
        }
        
        self.last_request.store(now, Ordering::Relaxed);
    }
}

pub static RATE_LIMITER: OnceLock<RateLimiter> = OnceLock::new();

pub fn get_rate_limiter() -> &'static RateLimiter {
    RATE_LIMITER.get_or_init(|| RateLimiter::new(10))
} 