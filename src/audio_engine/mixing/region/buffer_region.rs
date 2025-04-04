// buffer_region.rs
// Type of region that stores buffer data as a data.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::AudioSource;
use crate::audio_engine::Region;
use std::time::Duration;

pub struct BufferRegion {
    /// Start time of the region in frames
    start_time: Duration,
    /// Audio source of the region
    source: AudioSource,
}

impl BufferRegion {
    pub fn new(source: AudioSource) -> Self {
        Self {
            start_time: Duration::ZERO,
            source,
        }
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
        Duration::from_secs_f64(self.source.samples() as f64 / self.source.sample_rate as f64)
    }

    fn audio_source(&self) -> &AudioSource {
        &self.source
    }
}
