// region.rs
// A trait that represents a region in the track.
// © 2025 Shuntaro Kasatani

use crate::audio_utils::Beats;
use std::any::Any;

pub trait Region: Send + Sync + Any {
    /// Returns the start time of the region in f32.
    fn start_time(&self) -> Beats;

    /// Sets the start time of the region in beats.
    fn set_start_time(&mut self, start_time: Beats);

    /// Sets the duration of the region in beats.
    fn set_duration(&mut self, duration: Beats);

    /// Returns the end time of the region in beats.
    fn end_time(&self) -> Beats;

    /// Returns the duration of the region in beats.
    fn duration(&self) -> Beats;

    /// Sets the name of the region.
    fn set_name(&mut self, name: String);

    /// Returns the name of the region.
    fn name(&self) -> &str;

    /// Sets the id of the region.
    fn set_id(&mut self, id: u32);

    /// Returns the id of the region.
    fn id(&self) -> &u32;

    /// Returns whether the region overlaps with the given area.
    fn is_active_at(&self, start: Beats, end: Beats) -> bool {
        // Check if the chunk overlaps with the region.
        (start >= self.start_time() && start <= self.end_time())
            || (end >= self.start_time() && end <= self.end_time())
    }

    /// Returns the type of the region as a string.
    fn region_type(&self) -> String;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
