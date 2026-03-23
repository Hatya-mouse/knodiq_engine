use crate::{
    data_types::{AudioContext, Voice},
    graph::{Graph, error::GraphError},
    node::{
        Node,
        builtin::{AudioOutputNode, NoteInputNode},
    },
};
use cpal::{
    BufferSize, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use std::{
    sync::{Arc, Mutex},
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

        // Create a graph
        let input_node = NoteInputNode::default();
        let output_node = AudioOutputNode::default();
        let mut graph = Graph::new(
            Box::new(input_node),
            Box::new(output_node),
            audio_ctx.clone(),
        );

        // Add the node to the graph
        let node_id = graph.add_node(Box::new(node));
        // Connect the node
        let graph_input_id = graph.get_input_id();
        let graph_output_id = graph.get_output_id();
        graph.connect(&graph_input_id, "notes", &node_id, node_input_name)?;
        graph.connect(&node_id, node_output_name, &graph_output_id, "audio")?;
        // Prepare the graph
        graph.prepare()?;

        // Calculate the note array size
        let note_array_size = (audio_ctx.max_voices * audio_ctx.buffer_size) as usize;

        let off_note = Voice {
            frequency: 0.0,
            velocity: 1.0,
            is_active: false,
        };
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
                    graph.process(
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
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 86, 1.0, true, 950);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 0, 0.0, false, 50);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 86, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 84, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 81, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 79, 1.0, true, 1200);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 0, 0.0, false, 50);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 81, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 79, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 77, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 74, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 72, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 74, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 77, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 72, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 74, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 81, 1.0, true, 700);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 0, 0.0, false, 50);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 72, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 74, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 77, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 72, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 74, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 81, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 79, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 81, 1.0, true, 250);
        AudioPlayer::play_key(&audio_ctx, note_array_size, &notes, 0, 0.0, false, 1000);

        Ok(())
    }

    fn play_key(
        audio_ctx: &AudioContext,
        note_array_size: usize,
        notes: &Arc<Mutex<Vec<Voice>>>,
        note_number: u8,
        velocity: f32,
        is_active: bool,
        duration: u64,
    ) {
        // Create a kasl note
        let off_note = Voice {
            frequency: 0.0,
            velocity: 1.0,
            is_active: false,
        };

        // Calculate the frequency
        let frequency = 440.0 * 2.0f32.powf((f32::from(note_number) - 69.0) / 12.0);

        let mut on_notes = vec![off_note.clone(); note_array_size];
        for i in (0..note_array_size).step_by(audio_ctx.max_voices as usize) {
            on_notes[i] = Voice {
                frequency,
                velocity,
                is_active,
            };
        }
        *notes.lock().unwrap() = on_notes;
        thread::sleep(Duration::from_millis(duration));
    }
}
