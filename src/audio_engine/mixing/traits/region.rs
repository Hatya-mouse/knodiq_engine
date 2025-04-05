// region.rs
// A trait that represents a region in the track.
// © 2025 Shuntaro Kasatani

use std::time::Duration;

pub trait Region: Send + Sync {
    /// Returns the start time of the region std::time::Duration.
    fn start_time(&self) -> Duration;

    /// Returns the end time of the region std::time::Duration.
    fn end_time(&self) -> Duration;

    /// Returns the duration of the region std::time::Duration.
    fn duration(&self) -> Duration;

    /// Returns whether the region is active at the given area.
    fn is_active_at(&self, playhead: Duration, chunk_size: usize, sample_rate: usize) -> bool;
}
