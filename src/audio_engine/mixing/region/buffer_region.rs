// buffer_region.rs
// Type of region that stores buffer data as a data.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::{AudioSource, Duration, Region};

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

    fn end_time(&self) -> Duration {
        self.start_time + self.duration()
    }

    fn duration(&self) -> Duration {
        // Convert the number of samples to std::time::Duration.
        Duration::from_secs_f64(self.source.samples() as f64 / self.source.sample_rate as f64)
    }

    fn is_active_at(&self, playhead: Duration, chunk_size: usize, sample_rate: usize) -> bool {
        // Calculate the chunk's duration.
        let chunk_duration = Duration::from_secs_f64(chunk_size as f64 / sample_rate as f64);
        // Then calculate the chunk's end time.
        let chunk_end = playhead + chunk_duration;

        // Check if the chunk overlaps with the region.
        self.start_time < chunk_end && self.end_time() > playhead
    }
}
