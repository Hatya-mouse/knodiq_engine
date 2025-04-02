use crate::audio_engine::source::AudioSource;

pub trait Track {
    /// Returns the unique identifier of the track.
    fn id(&self) -> u32;

    /// Returns the name of the track.
    fn name(&self) -> &str;

    /// Sets the name of the track.
    fn set_name(&mut self, name: &str);

    /// Returns the current volume of the track.
    fn volume(&self) -> f32;

    /// Sets the volume of the track.
    fn set_volume(&mut self, volume: f32);

    /// Returns the sample rate of the track.
    fn sample_rate(&self) -> usize;

    /// Sets the sample rate of the track.
    fn set_sample_rate(&mut self, sample_rate: usize);

    /// Renders the audio source of the track.
    fn render(&mut self);

    /// Returns the rendered audio source.
    fn rendered_data(&self) -> Result<&AudioSource, Box<dyn std::error::Error>>;
}
