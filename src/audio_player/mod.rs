use std::{thread, time};

use crate::{audio_context::AudioContext, node::Node};
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

        // Play the sound
        let stream = self
            .device
            .build_output_stream(
                &config,
                move |data, _| {
                    node.process(&[], &[data.as_mut_ptr()], &audio_ctx);
                },
                |err| {
                    eprintln!("An error occured on stream: {}", err);
                },
                None,
            )
            .expect("Failed to create a new stream");
        stream.play().expect("Failed to play the stream");

        // Wait for the passed milliseconds
        thread::sleep(time::Duration::from_millis(duration));
    }
}
