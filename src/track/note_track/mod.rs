mod note;
mod note_region;
mod voice_event;

pub use note::{Note, NoteID};
pub use note_region::NoteRegion;

use crate::{
    data_types::{AudioContext, Voice},
    graph::{Graph, error::GraphError},
    mixer::TempoMap,
    node::builtin::{AudioOutputNode, NoteInputNode},
    track::{RegionID, Track},
};
use std::collections::{HashMap, VecDeque};
use voice_event::VoiceEvent;

#[derive(Default, Clone)]
pub struct NoteTrack {
    // --- GRAPH ---
    graph: Graph,

    // --- NOTE DATA ---
    regions: HashMap<RegionID, NoteRegion>,

    // --- VOICE MANAGEMENT ---
    events: Vec<VoiceEvent>,
    event_cursor: usize,
    active_voices: VecDeque<(usize, f32)>,
    free_voices: Vec<usize>,
    last_voices: Vec<Voice>,
    voice_buffer: Vec<Voice>,

    // --- AUDIO CONTEXT ---
    audio_ctx: AudioContext,

    // --- MISC ---
    next_region_id: usize,
}

impl NoteTrack {
    pub fn new(audio_ctx: AudioContext) -> Self {
        // Create a graph with the input and output nodes
        let input_node = NoteInputNode::default();
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

    pub fn add_region(&mut self, region: NoteRegion) -> RegionID {
        let id = self.generate_region_id();
        self.regions.insert(id, region);
        id
    }

    // --- VOICE GETTING ---

    /// Returns the vacant voice index, or returns the index of the oldest voice.
    fn find_or_steal_voice(&mut self, new_freq: f32) -> usize {
        let new_voice_index = self
            .free_voices
            .pop()
            .or_else(|| self.active_voices.pop_front().map(|v| v.0))
            .unwrap_or_default();
        self.active_voices.push_back((new_voice_index, new_freq));
        new_voice_index
    }
}

impl Track for NoteTrack {
    // --- CLONING ---

    fn clone_box(&self) -> Box<dyn Track> {
        Box::new(self.clone())
    }

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

    fn seek(&mut self) {
        // Clear all voices before seeking
        self.active_voices.clear();
        self.free_voices = (0..self.audio_ctx.max_voices).collect();
        self.last_voices = vec![Voice::default(); self.audio_ctx.max_voices];
    }

    // --- TRACK PROCESSING ---

    fn prepare(
        &mut self,
        _start: usize,
        _duration: usize,
        tempo_map: &TempoMap,
    ) -> Result<(), GraphError> {
        // Clear the old events
        self.events.clear();

        // Retrieve the notes from the regions in the track
        for region in self.regions.values() {
            // Calculate the start sample of the region
            for note in region.notes.values() {
                // Calculate the start and end sample of the note
                let start_sample = tempo_map.beats_to_samples(region.start + note.start);
                let end_sample =
                    tempo_map.beats_to_samples(region.start + note.start + note.duration);
                // Add the note start and end event to the events
                self.events.push(VoiceEvent::new(
                    start_sample,
                    note.frequency,
                    note.velocity,
                    true,
                ));
                self.events.push(VoiceEvent::new(
                    end_sample,
                    note.frequency,
                    note.velocity,
                    false,
                ));
            }
        }

        // Sort the events
        self.events.sort_unstable_by_key(|e| e.sample_index);

        // Initialize the voice buffer
        self.voice_buffer =
            vec![Voice::default(); self.audio_ctx.buffer_size * self.audio_ctx.max_voices];

        // Initialize the voices
        self.active_voices.clear();
        self.free_voices = (0..self.audio_ctx.max_voices).collect();
        self.last_voices = vec![Voice::default(); self.audio_ctx.max_voices];

        // Prepare the graph
        self.graph.prepare()
    }

    fn process(&mut self, playhead: usize, output: &mut [f32]) {
        // Convert the playhead beats to samples
        let buffer_end = playhead + self.audio_ctx.buffer_size;
        let max_voices = self.audio_ctx.max_voices;

        // Seek the event cursor
        if self
            .events
            .get(self.event_cursor)
            .is_some_and(|e| e.sample_index > playhead)
            || (self.event_cursor > 0 && self.events[self.event_cursor - 1].sample_index > playhead)
        {
            self.event_cursor = self.events.partition_point(|e| e.sample_index < playhead);
        }

        for sample in playhead..buffer_end {
            // Calculate the local sample in the buffer chunk
            let local_sample = sample - playhead;
            // Calculate the index of the first current voice
            let current = local_sample * max_voices;

            // If the current sample is the first sample in the buffer,
            // Copy from the last voices
            if local_sample == 0 && !self.last_voices.is_empty() {
                self.voice_buffer[..max_voices].clone_from_slice(&self.last_voices);
            }

            // If the current sample is not the first sample in the buffer,
            // copy the previous voices to the current index
            if local_sample > 0 {
                let previous = (local_sample - 1) * max_voices;
                // Get a mutable slice from the voice buffer
                let (prev_slice, curr_slice) = self.voice_buffer.split_at_mut(current);
                // Copy the previous slice to the mutable slice of the current buffer
                curr_slice[..max_voices]
                    .clone_from_slice(&prev_slice[previous..previous + max_voices]);
            }

            // Increment the elapsed_samples
            for (index, _) in self.active_voices.iter() {
                self.voice_buffer[current + index].age += 1.0 / self.audio_ctx.sample_rate as f32;
            }

            // Consume the events in this sample
            while let Some(event) = self.events.get(self.event_cursor) {
                // Break if the event's sample index is not current sample
                if event.sample_index != sample {
                    break;
                }

                // Copy the frequency and velocity to avoid reference issues
                let frequency = event.frequency;
                let velocity = event.velocity;

                if event.is_note_on {
                    // Start playing the note from the sample
                    let voice_index = self.find_or_steal_voice(frequency);
                    // Set the new voice to the voice buffer
                    self.voice_buffer[current + voice_index] =
                        Voice::new(frequency, velocity, 0.0, true);
                } else {
                    // Remove the active voice whose frequency matches the event frequency
                    if let Some(remove_index) = self
                        .active_voices
                        .iter()
                        .position(|(_, freq)| *freq == event.frequency)
                    {
                        // Remove the index from the active_voices and get the voice index
                        let (voice_index, _) = self.active_voices.remove(remove_index).unwrap();
                        // Mark the voice index as free
                        self.free_voices.push(voice_index);
                        self.voice_buffer[current + voice_index].is_active = false;
                        self.voice_buffer[current + voice_index].age = 0.0;
                    }
                }

                // Increment the event cursor
                self.event_cursor += 1;
            }
        }

        // Copy the last voices
        let last = (self.audio_ctx.buffer_size - 1) * max_voices;
        self.last_voices
            .clone_from_slice(&self.voice_buffer[last..last + max_voices]);

        // Get a pointer to the voice buffer
        let input_ptr = self.voice_buffer.as_ptr() as *const u8;
        // Process the graph
        self.graph
            .process(&[input_ptr], &[output.as_mut_ptr() as *mut u8]);
    }

    // --- ANY CASTING ---

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
