use crate::audio_engine::source::AudioSource;
use std::time::Duration;

pub trait Region: Send + Sync {
    /// Returns the start time of the region std::time::Duration.
    fn start_time(&self) -> Duration;

    /// Returns the end time of the region std::time::Duration.
    fn end_time(&self) -> Duration;

    /// Returns the duration of the region std::time::Duration.
    fn duration(&self) -> Duration;

    /// Returns the audio stream of the region.
    fn audio_source(&self) -> &AudioSource;
}
