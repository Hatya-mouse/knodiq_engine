// region.rs
// A trait that represents a region in the track.
// © 2025 Shuntaro Kasatani

use std::time::Duration;

pub trait Region: Send + Sync {
    /// Returns the start time of the region std::time::Duration.
    fn start_time(&self) -> f32;

    /// Sets the start time of the region std::time::Duration.
    fn set_start_time(&mut self, start_time: f32);

    /// Returns the end time of the region std::time::Duration.
    fn end_time(&self) -> f32;

    /// Returns the duration of the region std::time::Duration.
    fn duration(&self) -> f32;

    /// Returns whether the region overlaps with the given area.
    fn is_active_at(&self, start: f32, end: f32) -> bool {
        // Check if the chunk overlaps with the region.
        (start >= self.start_time() && start <= self.end_time())
            || (end >= self.start_time() && end <= self.end_time())
    }
}
