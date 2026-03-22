use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{audio_context::AudioContext, node::Node, note::KaslNote};
use cpal::{
    BufferSize, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

pub struct AudioPlayer {
    device: cpal::Device,
}

impl AudioPlayer {
    pub fn new(device_id: Option<String>) -> Self {
        let host = cpal::default_host();
        let id: Option<cpal::DeviceId> =
            device_id.map(|id| id.parse().expect("Failed to parse the device ID"));
        let device = id
            .as_ref()
            .map_or_else(|| host.default_output_device(), |id| host.device_by_id(id))
            .expect("Failed to find the output device");

        Self { device }
    }

    pub fn play_audio<N>(&self, audio_ctx: AudioContext, mut node: N, duration: u64)
    where
        N: Node + 'static,
    {
        // Create a config
        let config = StreamConfig {
            channels: audio_ctx.channels,
            sample_rate: audio_ctx.sample_rate,
            buffer_size: BufferSize::Fixed(audio_ctx.buffer_size),
        };

        // Prepare the node
        node.prepare(&audio_ctx);

        // Create a kasl note
        let off_note = KaslNote {
            frequency: 440.0,
            velocity: 0.01,
            is_active: false,
        };

        // Calculate the note array size
        let note_array_size = (audio_ctx.max_voices * audio_ctx.buffer_size) as usize;

        let notes = Arc::new(Mutex::new(vec![off_note.clone(); note_array_size]));
        let notes_clone = Arc::clone(&notes);

        // Clone the audio_ctx
        let moved_ctx = audio_ctx.clone();

        // Play the sound
        let stream = self
            .device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    let notes = notes_clone.lock().unwrap();
                    node.process(
                        &[notes.as_ptr() as *const u8],
                        &[data.as_mut_ptr() as *mut u8],
                        &moved_ctx,
                    );
                },
                |err| {
                    eprintln!("An error occured on stream: {}", err);
                },
                None,
            )
            .expect("Failed to create a new stream");
        stream.play().expect("Failed to play the stream");

        // Wait for the passed milliseconds
        let mut on_notes = vec![off_note.clone(); note_array_size];
        for i in (0..note_array_size).step_by(audio_ctx.max_voices as usize) {
            on_notes[i] = KaslNote {
                frequency: 440.0,
                velocity: 0.3,
                is_active: true,
            };
            on_notes[i + 1] = KaslNote {
                frequency: 550.0,
                velocity: 0.3,
                is_active: true,
            };
            on_notes[i + 2] = KaslNote {
                frequency: 660.0,
                velocity: 0.3,
                is_active: true,
            };
        }
        *notes.lock().unwrap() = on_notes;
        thread::sleep(Duration::from_millis(duration));
        *notes.lock().unwrap() = vec![off_note.clone(); note_array_size];
        thread::sleep(Duration::from_millis(duration));
        let mut on_notes = vec![off_note.clone(); note_array_size];
        for i in (0..note_array_size).step_by(audio_ctx.max_voices as usize) {
            on_notes[i] = KaslNote {
                frequency: 440.0,
                velocity: 0.5,
                is_active: true,
            };
        }
        *notes.lock().unwrap() = on_notes;
        thread::sleep(Duration::from_millis(duration));
    }
}
