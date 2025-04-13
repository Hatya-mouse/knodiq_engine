// buffer_region.rs
// Type of region that stores buffer data as a data.
// © 2025 Shuntaro Kasatani

use crate::{AudioSource, Duration, Region};

pub struct BufferRegion {
    /// Start time of the region in frames.
    start_time: Duration,
    /// Audio source of the region.
    source: AudioSource,
}

impl BufferRegion {
    /// Creates a new buffer region with the given audio source.
    pub fn new(source: AudioSource) -> Self {
        Self {
            start_time: Duration::ZERO,
            source,
        }
    }

    /// Returns the audio source of the region.
    pub fn audio_source(&self) -> &AudioSource {
        &self.source
    }

    /// Sets the audio source of the region.
    pub fn set_audio_source(&mut self, source: AudioSource) {
        self.source = source;
    }
}

impl Region for BufferRegion {
    fn start_time(&self) -> Duration {
        self.start_time
    }

    fn set_start_time(&mut self, start_time: Duration) {
        self.start_time = start_time;
    }

    fn end_time(&self) -> Duration {
        self.start_time + self.duration()
    }

    fn duration(&self) -> Duration {
        // Convert the number of samples to std::time::Duration.
        Duration::from_secs_f64(self.source.samples() as f64 / self.source.sample_rate as f64)
    }

    fn is_active_at(&self, start: Duration, end: Duration) -> bool {
        // Check if the chunk overlaps with the region.
        (start >= self.start_time && start <= self.end_time())
            || (end >= self.start_time && end <= self.end_time())
    }
}
