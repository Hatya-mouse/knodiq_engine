mod audio_region;
mod resampler;
mod tempo_strech;

pub use audio_region::AudioRegion;

use crate::{
    data_types::AudioContext,
    graph::{Graph, error::GraphError},
    mixer::TempoMap,
    node::builtin::{AudioInputNode, AudioOutputNode},
    track::{RegionID, Track, audio_track::tempo_strech::tempo_strech},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct AudioTrack {
    // --- GRAPH ---
    graph: Graph,

    // --- RAW AUDIO DATA ---
    regions: HashMap<RegionID, AudioRegion>,
    processed: Vec<f32>,

    // --- AUDIO CONTEXT ---
    audio_ctx: AudioContext,

    // --- MISC ---
    next_region_id: usize,
}

impl AudioTrack {
    pub fn new(audio_ctx: AudioContext) -> Self {
        // Create a graph with the input and output nodes
        let input_node = AudioInputNode::default();
        let output_node = AudioOutputNode::default();
        let graph = Graph::new(
            Box::new(input_node),
            Box::new(output_node),
            audio_ctx.clone(),
        );

        Self {
            graph,
            audio_ctx,
            ..Default::default()
        }
    }

    // --- REGION ADDITION ---

    fn generate_region_id(&mut self) -> RegionID {
        let id = RegionID(self.next_region_id);
        self.next_region_id += 1;
        id
    }

    pub fn add_region(&mut self, region: AudioRegion) {
        let id = self.generate_region_id();
        self.regions.insert(id, region);
    }
}

impl Track for AudioTrack {
    // --- GRAPH GETTING ---

    fn get_graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    // --- AUDIO CONTEXT UPDARING ---

    fn set_audio_ctx(&mut self, audio_ctx: &AudioContext) {
        self.audio_ctx = audio_ctx.clone();
        self.graph.set_audio_ctx(audio_ctx);
    }

    // --- SEEKING ---

    fn seek(&mut self) {}

    // --- TRACK PROCESSING ---

    fn prepare(
        &mut self,
        _start: usize,
        duration: usize,
        tempo_map: &TempoMap,
    ) -> Result<(), GraphError> {
        // Calculate the total sample number
        // Ceil to a multiple of the buffer size
        let total_frames =
            duration.div_ceil(self.audio_ctx.buffer_size) * self.audio_ctx.buffer_size;
        // Initialize the processed vector with zeros
        self.processed = vec![0.0; total_frames * self.audio_ctx.channels];

        // Resample the each regions
        for region in self.regions.values() {
            let resampled = tempo_strech(
                region,
                self.audio_ctx.sample_rate,
                self.audio_ctx.channels,
                tempo_map,
            );

            // Calculate the start sample index of the buffer
            let region_start_index = tempo_map.beats_to_samples(region.start);

            // Add the resampled samples
            let available = self.processed.len().saturating_sub(region_start_index);
            let copy_len = resampled.len().min(available);
            for (i, sample) in resampled[..copy_len].iter().enumerate() {
                self.processed[region_start_index + i] += sample;
            }
        }

        // Then prepare the graph
        self.graph.prepare()
    }

    fn process(&mut self, playhead: usize, output: *mut u8) {
        let buffer_end = playhead + self.audio_ctx.buffer_size * self.audio_ctx.channels;
        // Get the slice pointer from the processed audio data
        let input_ptr = if buffer_end <= self.processed.len() {
            self.processed[playhead..buffer_end].as_ptr() as *const u8
        } else {
            std::ptr::null()
        };
        // Process the graph
        self.graph.process(&[input_ptr], &[output]);
    }
}
