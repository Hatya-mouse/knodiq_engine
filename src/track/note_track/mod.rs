mod note_region;
mod voice_event;

pub use note_region::{Note, NoteRegion};

use crate::{
    data_types::{AudioContext, Beats},
    graph::{Graph, error::GraphError},
    track::{RegionID, Track},
};
use std::collections::HashMap;
use voice_event::VoiceEvent;

pub struct NoteTrack {
    // --- GRAPH ---
    graph: Graph,

    // --- NOTE DATA ---
    regions: HashMap<RegionID, NoteRegion>,
    events: Vec<VoiceEvent>,

    // --- AUDIO CONTEXT ---
    audio_ctx: AudioContext,

    // --- MISC ---
    next_region_id: usize,
}

impl NoteTrack {
    // --- NUMBER CONVERSION ---

    fn beats_to_samples(&self, beats: Beats) -> usize {
        (beats.0 / self.audio_ctx.tempo as f64 * 60.0 * self.audio_ctx.sample_rate as f64) as usize
            * self.audio_ctx.channels as usize
    }

    // --- REGION ADDITION ---

    fn generate_region_id(&mut self) -> RegionID {
        let id = RegionID(self.next_region_id);
        self.next_region_id += 1;
        id
    }

    fn add_region(&mut self, region: NoteRegion) {
        let id = self.generate_region_id();
        self.regions.insert(id, region);
    }
}

impl Track for NoteTrack {
    // --- AUDIO CONTEXT UPDARING ---

    fn set_audio_ctx(&mut self, audio_ctx: &AudioContext) {
        self.audio_ctx = audio_ctx.clone();
        self.graph.set_audio_ctx(audio_ctx);
    }

    // --- TRACK PROCESSING ---

    fn prepare(&mut self, _total_duration: Beats) -> Result<(), GraphError> {
        // Clear the old events
        self.events.clear();

        // Retrieve the notes from the regions in the track
        for region in self.regions.values() {
            // Calculate the start sample of the region
            let region_start_sample = self.beats_to_samples(region.start);
            for note in &region.notes {
                // Calculate the start and end sample of the note
                let start_sample = region_start_sample + self.beats_to_samples(note.start);
                let end_sample =
                    region_start_sample + self.beats_to_samples(note.start + note.duration);
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

        // Prepare the graph
        self.graph.prepare()
    }

    fn process(&mut self, playhead: Beats, output: *mut u8, audio_ctx: &AudioContext) {
        // Process the graph
        // self.graph.process(&[input_ptr], &[output], audio_ctx);
    }
}
