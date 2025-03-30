use crate::audio_engine::source::AudioSource;
use rodio::{OutputStream, Sink};
use std::error::Error;

pub struct AudioPlayer {
    /// Audio source loaded into the player.
    source: Option<AudioSource>,
    /// Sink used to play audio.
    sink: Sink,
    /// Volume of the playback.
    volume: f32,
}

impl AudioPlayer {
    /// Create a new audio player instance.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        Ok(Self {
            source: None,
            sink: Sink::try_new(&stream_handle)?,
            volume: 1.0,
        })
    }

    /// Load an audio source into the player.
    pub fn load_source(&mut self, source: AudioSource) -> Result<(), Box<dyn Error>> {
        self.source = Some(source);
        Ok(())
    }

    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        match &self.source {
            Some(source) => self.sink.append((*source).clone()),
            None => return Err("No audio source loaded".into()),
        }
        Ok(())
    }

    pub fn wait_for_finish(&self) {
        self.sink.sleep_until_end();
    }
}
