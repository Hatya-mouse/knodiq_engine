// duration.rs
// Converts between std::time::Duration and sample count
// © 2025 Shuntaro Kasatani

use std::time::Duration;

/// Converts a duration to sample count
pub fn as_samples(sample_rate: usize, duration: Duration) -> usize {
    (duration.as_secs_f64() * sample_rate as f64) as usize
}

/// Converts sample count to duration
pub fn as_duration(sample_rate: usize, samples: usize) -> Duration {
    Duration::from_secs_f64(samples as f64 / sample_rate as f64)
}
