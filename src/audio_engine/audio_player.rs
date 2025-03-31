// audio_player.rs
// Audio player for playing audio sources.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::source::AudioSource;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{mpsc, Arc, Mutex};

pub struct AudioPlayer {
    /// Audio source loaded into the player.
    source: Option<AudioSource>,
    /// Audio stream used to play the audio source.
    stream: Option<cpal::Stream>,
    /// A mspc receiver to know when the audio stream has finished playback.
    receiver: Option<mpsc::Receiver<()>>,
    /// Playback completion handler.
    playback_completion: Option<Box<dyn FnOnce() + Send + 'static>>,
    /// Current playback duration.
    pub frame_index: Arc<Mutex<usize>>,
    /// Volume of the playback.
    pub volume: f32,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            source: None,
            stream: None,
            receiver: None,
            playback_completion: None,
            frame_index: Arc::new(Mutex::new(0)),
            volume: 1.0,
        }
    }

    pub fn load_source(&mut self, source: AudioSource) -> Result<(), Box<dyn std::error::Error>> {
        let (stream, receiver) = match self.create_stream(&source) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        // Set the stream
        self.stream = Some(stream);
        // Set the receiver
        self.receiver = Some(receiver);
        // Set the audio source
        self.source = Some(source);
        Ok(())
    }

    fn create_stream(
        &mut self,
        source: &AudioSource,
    ) -> Result<(cpal::Stream, mpsc::Receiver<()>), Box<dyn std::error::Error>> {
        // Create a playback stream from the audio source
        // First get the default host and device
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();

        // Get the config and set the sample rate
        let config = device.default_output_config()?;
        let mut stream_config = config.config();
        stream_config.sample_rate = cpal::SampleRate(source.sample_rate as u32);

        // Create a sync channel to know when the stream has finished playback
        let (sender, receiver) = mpsc::channel();

        // Copy the audio data
        let owned_data = source.data.to_owned();
        // Number of channels in the audio source
        let channels = source.channels;
        // Number of samples in the audio source
        let total_samples = source.samples();
        // Clone the current frame index
        let frame_index = Arc::clone(&self.frame_index);
        // Volume reference
        let volume = self.volume;

        // Create a playback stream from the audio source
        match device.build_output_stream(
            &stream_config,
            move |data: &mut [f32], _| {
                // Lock the frame index
                let mut frame_index = frame_index.lock().unwrap();
                for sample in data.iter_mut() {
                    // Calculate the frame index
                    let frame = *frame_index / channels;
                    if frame < total_samples {
                        // Calculate the channel index
                        let channel = *frame_index % channels;
                        // Get the sample from the source
                        let owned_sample = owned_data[channel][frame];
                        // Apply the volume and pass the sample value
                        *sample = owned_sample * volume;
                        // Increment the frame index
                        *frame_index += 1;
                    } else {
                        // Notify that the playback has finished
                        let _ = sender.send(());
                    }
                }
            },
            move |err| {
                println!("Audio stream couldn't be initialized: {}", err);
            },
            None,
        ) {
            Ok(stream) => Ok((stream, receiver)),
            Err(err) => Err(err.into()),
        }
    }

    pub fn play(
        &mut self,
        completion: Option<Box<dyn FnOnce() + Send + 'static>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &self.stream {
            Some(s) => s.play()?,
            None => return Err("No stream available".into()),
        }
        self.playback_completion = completion;
        Ok(())
    }

    pub fn wait_for_finish(&mut self) {
        if let Some(receiver) = &mut self.receiver {
            receiver.recv().unwrap();
            // Run the completion handler
            if let Some(completion) = self.playback_completion.take() {
                completion();
            }
        }
    }
}
