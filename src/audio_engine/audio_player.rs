// audio_player.rs
// Audio player for playing audio sources.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::{AudioSource, Mixer};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::queue::SegQueue;
use std::sync::{mpsc, mpsc::TryRecvError, Arc, Mutex};

pub struct AudioPlayer {
    /// Currently playing audio source.
    playing_source: Option<Arc<Mutex<AudioSource>>>,

    /// Currently playing stream.
    current_stream: Option<cpal::Stream>,

    /// A mspc receiver to know when the audio stream has finished playback.
    receiver: Option<mpsc::Receiver<()>>,

    /// Sample rate of the audio player.
    pub sample_rate: usize,

    /// Channels of the audio player.
    pub channels: usize,

    /// Playback completion handler
    pub completion_handler: Option<Box<dyn FnOnce()>>,

    /// Current playback duration.
    pub frame_index: Arc<Mutex<usize>>,

    /// Volume of the playback.
    pub volume: f32,

    /// Playback queue.
    pub audio_queue: SegQueue<f32>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            playing_source: None,
            current_stream: None,
            receiver: None,
            sample_rate: 44100,
            channels: 2,
            completion_handler: None,
            frame_index: Arc::new(Mutex::new(0)),
            volume: 1.0,
            audio_queue: SegQueue::new(),
        }
    }

    /// Add an audio buffer data to the end of the currently playing source.
    /// The first audio source's sample rate and channels will be used to create a audio stream.
    pub fn add_queue(&mut self, source: &AudioSource) -> Result<(), Box<dyn std::error::Error>> {
        // Get the buffer data
        let buffer = &source.data.clone();

        // set the source
        match self.playing_source {
            Some(ref mut playing_source) => {
                for channel in 0..self.channels {
                    let extend_data = &buffer[channel];
                    // unwrap the playing source
                    let playing_source = Arc::clone(playing_source);
                    let mut locked_source = playing_source.lock().unwrap();
                    locked_source.data[channel].extend(extend_data);
                }
            }
            None => {
                self.channels = source.channels;
                self.sample_rate = source.sample_rate;
                self.playing_source = Some(Arc::new(Mutex::new(source.clone())));
            }
        }

        if self.current_stream.is_none() {
            let (stream, receiver) = self.create_stream()?;
            self.receiver = Some(receiver);
            // Play the stream
            stream.play()?;
            // Set the current stream
            self.current_stream = Some(stream);
        }
        Ok(())
    }

    /// Create a playback stream from the audio source.
    fn create_stream(
        &mut self,
    ) -> Result<(cpal::Stream, mpsc::Receiver<()>), Box<dyn std::error::Error>> {
        // Create a playback stream from the audio source
        // First get the default host and device
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();

        // Get the config and set the sample rate
        let config = device.default_output_config()?;
        let mut stream_config = config.config();
        stream_config.sample_rate.0 = self.sample_rate as u32;

        // Create a sync channel to know when the stream has finished playback
        let (sender, receiver) = mpsc::channel();

        // If the playing source is None, return an error
        if self.playing_source.is_none() {
            return Err("No audio source to play".into());
        }

        // Copy the audio source
        let playing_source = Arc::clone(self.playing_source.as_ref().unwrap());
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
                // Lock the audio source
                let locked_source = playing_source.lock().unwrap();
                for sample in data.iter_mut() {
                    // Calculate the frame index
                    let frame = *frame_index / locked_source.channels;
                    if frame < locked_source.samples() {
                        // Calculate the channel index
                        let channel = *frame_index % locked_source.channels;
                        // Check if the channel exists
                        if channel < locked_source.channels {
                            // Get the sample from the source
                            let owned_sample = locked_source.data[channel][frame];
                            // Apply the volume and pass the sample value
                            *sample = owned_sample * volume;
                            // Increment the frame index
                            *frame_index += 1;
                        }
                    } else {
                        // Notify that the playback has finished
                        let _ = sender.send(());
                        *frame_index = 0;
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

    pub fn update(&mut self) {
        if let Some(receiver) = &self.receiver {
            // Try to receive without blocking the main thread
            match receiver.try_recv() {
                Ok(()) => {
                    // Run the completion handler
                    if let Some(handler) = self.completion_handler.take() {
                        handler();
                    }
                    // Drop the source and the stream
                    drop(self.playing_source.take());
                    drop(self.current_stream.take());
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {}
            }
        }
    }

    pub fn enqueue_audio(&mut self, audio_data: f32) {
        self.audio_queue.push(audio_data);
    }
}
