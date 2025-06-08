// audio_player.rs
// Audio player for playing audio sources.
// © 2025 Shuntaro Kasatani

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;
use std::thread;

pub struct AudioPlayer {
    /// Currently playing stream.
    current_stream: Option<cpal::Stream>,

    /// Sample rate of the audio player.
    pub sample_rate: usize,

    /// Channels of the audio player.
    pub channels: usize,

    /// Volume of the playback.
    pub volume: f32,
}

impl AudioPlayer {
    pub fn new(sample_rate: usize) -> Self {
        Self {
            current_stream: None,
            sample_rate,
            channels: 2,
            volume: 1.0,
        }
    }

    /// Create a new audio player stream.
    ///
    /// # Return
    /// - `mpsc::Sender<f32>`: A channel to send the sample data asynchronously.
    pub fn initialize_player(
        &mut self,
        sample_rate: usize,
        channels: usize,
        mut completion_handler: Option<Box<dyn FnOnce() + Send>>,
    ) -> Result<mpsc::Sender<f32>, Box<dyn std::error::Error>> {
        self.channels = channels;
        self.sample_rate = sample_rate;

        let (stream, completion_receiver, sample_sender) = self.create_stream()?;
        // Play the stream
        stream.play()?;

        // Set the current stream
        self.current_stream = Some(stream);

        // Create a thread to handle the stream completion
        thread::spawn(move || {
            // Wait for the stream to finish playback
            match completion_receiver.recv() {
                Ok(_) => {
                    // Call the completion handler if it exists
                    if let Some(handler) = completion_handler.take() {
                        handler();
                    }
                }
                Err(err) => {
                    eprintln!("Couldn't receive the completion signal: {}", err);
                }
            }
        });

        // Return the sample sender
        Ok(sample_sender)
    }

    /// Create a playback stream from the audio source.
    ///
    /// # Return
    /// - `cpal::Stream`: The playback stream.
    /// - `mpsc::Receiver<()>`: A channel to receive completion signals.
    /// - `mpsc::Sender<f32>`: A channel to send sample value.
    fn create_stream(
        &mut self,
    ) -> Result<(cpal::Stream, mpsc::Receiver<()>, mpsc::Sender<f32>), Box<dyn std::error::Error>>
    {
        // Create a playback stream from the audio source
        // First get the default host and device
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();

        // Get the config and set the sample rate
        let config = device.default_output_config()?;
        let mut stream_config = config.config();
        stream_config.sample_rate.0 = self.sample_rate as u32;

        // Create a channel to know when the stream has finished playback
        let (completion_sender, completion_receiver) = mpsc::channel::<()>();

        // Create a channel to receive audio sample data asynchronously
        let (sample_sender, sample_receiver) = mpsc::channel::<f32>();

        // Volume reference
        let volume = self.volume;

        // Create a playback stream from the audio source
        match device.build_output_stream(
            &stream_config,
            move |data: &mut [f32], output_callback_info| {
                output_callback_info.timestamp();
                for sample_out in data.iter_mut() {
                    // Get the sample data from the receiver
                    let sample_data = match sample_receiver.recv() {
                        Ok(sample) => sample,
                        Err(_) => {
                            let _ = completion_sender.send(());
                            continue;
                        }
                    };
                    *sample_out = sample_data * volume;
                }
            },
            move |err| {
                eprintln!("Audio stream couldn't be initialized: {}", err);
            },
            None,
        ) {
            Ok(stream) => Ok((stream, completion_receiver, sample_sender)),
            Err(err) => Err(err.into()),
        }
    }

    pub fn pause(&mut self) -> Result<(), cpal::PauseStreamError> {
        if let Some(stream) = &self.current_stream {
            return stream.pause();
        }
        Ok(())
    }
}

unsafe impl Send for AudioPlayer {}
