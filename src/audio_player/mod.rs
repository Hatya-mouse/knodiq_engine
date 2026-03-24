use crate::{
    data_types::{AudioContext, Beats},
    graph::error::GraphError,
    node::Node,
    track::{
        Track,
        note_track::{Note, NoteRegion, NoteTrack},
    },
};
use cpal::{
    BufferSize, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread,
    time::Duration,
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

    pub fn play_audio<N>(
        &self,
        audio_ctx: AudioContext,
        node: N,
        node_input_name: &str,
        node_output_name: &str,
        duration: u64,
    ) -> Result<(), GraphError>
    where
        N: Node + 'static,
    {
        // Create a config
        let config = StreamConfig {
            channels: audio_ctx.channels,
            sample_rate: audio_ctx.sample_rate,
            buffer_size: BufferSize::Fixed(audio_ctx.buffer_size),
        };

        // Create a track
        let mut note_track = NoteTrack::new(audio_ctx.clone());

        // Add notes to the track
        let mut note_region = NoteRegion::new(Beats(0.0), Beats(17.0), Vec::new());
        note_region.add_note(Note::new(Beats(0.0), Beats(1.9), 1174.65, 1.0));
        note_region.add_note(Note::new(Beats(2.0), Beats(0.5), 1174.65, 1.0));
        note_region.add_note(Note::new(Beats(2.5), Beats(0.5), 1046.50, 1.0));
        note_region.add_note(Note::new(Beats(3.0), Beats(0.5), 880.00, 1.0));
        note_region.add_note(Note::new(Beats(3.5), Beats(2.4), 783.99, 1.0));

        note_region.add_note(Note::new(Beats(6.0), Beats(0.5), 880.00, 1.0));
        note_region.add_note(Note::new(Beats(6.5), Beats(0.5), 783.99, 1.0));
        note_region.add_note(Note::new(Beats(7.0), Beats(0.5), 698.45, 1.0));
        note_region.add_note(Note::new(Beats(7.5), Beats(0.5), 587.32, 1.0));

        note_region.add_note(Note::new(Beats(8.0), Beats(0.5), 523.25, 1.0));
        note_region.add_note(Note::new(Beats(8.5), Beats(0.5), 587.32, 1.0));
        note_region.add_note(Note::new(Beats(9.0), Beats(0.5), 698.45, 1.0));
        note_region.add_note(Note::new(Beats(9.5), Beats(0.5), 523.25, 1.0));
        note_region.add_note(Note::new(Beats(10.0), Beats(0.5), 587.32, 1.0));
        note_region.add_note(Note::new(Beats(10.5), Beats(1.4), 880.00, 1.0));

        note_region.add_note(Note::new(Beats(12.0), Beats(0.5), 523.25, 1.0));
        note_region.add_note(Note::new(Beats(12.5), Beats(0.5), 587.32, 1.0));
        note_region.add_note(Note::new(Beats(13.0), Beats(0.5), 698.45, 1.0));
        note_region.add_note(Note::new(Beats(13.5), Beats(0.5), 523.25, 1.0));
        note_region.add_note(Note::new(Beats(14.0), Beats(0.5), 587.32, 1.0));
        note_region.add_note(Note::new(Beats(14.5), Beats(0.5), 880.00, 1.0));
        note_region.add_note(Note::new(Beats(15.0), Beats(0.5), 783.99, 1.0));
        note_region.add_note(Note::new(Beats(15.5), Beats(0.5), 880.00, 1.0));

        // Add the region to the track
        note_track.add_region(note_region);

        // Get the graph from the track
        let graph = note_track.get_graph_mut();
        // Add the node to the graph
        let node_id = graph.add_node(Box::new(node));
        // Connect the node
        let graph_input_id = graph.get_input_id();
        let graph_output_id = graph.get_output_id();
        graph.connect(&graph_input_id, "notes", &node_id, node_input_name)?;
        graph.connect(&node_id, node_output_name, &graph_output_id, "audio")?;

        // Prepare the track
        note_track.prepare(Beats(17.0))?;

        // Play the sound
        let playhead_samples = Arc::new(AtomicU64::new(0));
        let playhead_clone = playhead_samples.clone();

        let stream = self
            .device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    println!(
                        "actual buffer size: {}",
                        data.len() / audio_ctx.channels as usize
                    );
                    let sample = playhead_clone.load(Ordering::Relaxed);
                    let beats = Beats(
                        sample as f64 / audio_ctx.sample_rate as f64 / 60.0
                            * audio_ctx.tempo as f64,
                    );
                    note_track.process(beats, data.as_mut_ptr() as *mut u8, &audio_ctx);
                    playhead_clone.fetch_add(audio_ctx.buffer_size as u64, Ordering::Relaxed);
                },
                |err| {
                    eprintln!("An error occured on stream: {}", err);
                },
                None,
            )
            .expect("Failed to create a new stream");
        stream.play().expect("Failed to play the stream");

        thread::sleep(Duration::from_millis(duration));

        Ok(())
    }
}
