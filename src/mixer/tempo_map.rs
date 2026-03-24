use crate::{
    data_types::{AudioContext, Beats},
    mixer::TempoEvent,
};

pub struct TempoMap {
    pub events: Vec<TempoEvent>,
    audio_ctx: AudioContext,
}

impl TempoMap {
    // --- NEW ---

    /// Creates a new TempoMap.
    pub fn new(audio_ctx: AudioContext, initial_bpm: f64) -> Self {
        Self {
            events: vec![TempoEvent {
                beat: Beats(0.0),
                bpm: initial_bpm,
                sample_offset: 0,
            }],
            audio_ctx,
        }
    }

    // --- AUDIO CONTEXT ---

    /// Sets the audio context to the new one and calculate the sample offsets of all events in the TempoMap.
    pub fn set_audio_ctx(&mut self, audio_ctx: AudioContext) {
        self.audio_ctx = audio_ctx;
        // Calculate the offsets of all events
        self.calculate_sample_offsets(0);
    }

    // --- TEMPO EVENT MANAGEMENT ---

    /// Adds a new tempo event to the tempo map.
    pub fn add_event(&mut self, event: TempoEvent) {
        // Insert the event while preserving the order
        let index = match self.events.binary_search(&event) {
            Ok(index) => {
                // Overwrite the old event if the event with the same beat exists
                self.events.remove(index);
                self.events.insert(index, event);
                index
            }
            Err(index) => {
                self.events.insert(index, event);
                index
            }
        };

        // Calculate the sample offsets of the events after the modified index
        self.calculate_sample_offsets(index);
    }

    /// Removes the tempo event from the tempo map.
    pub fn remove_event(&mut self, index: usize) {
        // Cannot return the event with the zero beats
        if index == 0 {
            return;
        }

        // Ensure that the TempoMap has at least one event
        if self.events.len() > 1 {
            // Remove the event in the index
            self.events.remove(index);
            // Calculate the sample offsets of the events after the removed index
            self.calculate_sample_offsets(index);
        }
    }

    /// Change the BPM of the event in the specified index.
    pub fn change_bpm(&mut self, index: usize, bpm: f64) {
        // Get a mutable reference to the target event
        let Some(event) = self.events.get_mut(index) else {
            return;
        };

        // Update the bpm
        event.bpm = bpm;

        // Calculate the sample offsets of the events after the event
        self.calculate_sample_offsets(index);
    }

    // --- OFFSET CALCULATION ---

    /// Calculate the offsets of the events after the given index.
    fn calculate_sample_offsets(&mut self, after_index: usize) {
        for i in after_index..self.events.len() {
            if i == 0 {
                self.events[i].sample_offset = 0;
            } else {
                let prev = &self.events[i - 1];
                let beat_diff = self.events[i].beat - prev.beat;
                let samples =
                    (beat_diff.0 / prev.bpm * 60.0 * self.audio_ctx.sample_rate as f64) as usize;
                self.events[i].sample_offset = prev.sample_offset + samples;
            }
        }
    }

    // --- BEATS CONVERSION ---

    /// Convert the Beats to samples using the tempo map.
    pub fn beats_to_samples(&self, beats: Beats) -> usize {
        let idx = self.events.partition_point(|e| e.beat <= beats) - 1;
        let event = &self.events[idx];
        let remaining_beats = beats - event.beat;
        event.sample_offset
            + (remaining_beats.0 / event.bpm * 60.0 * self.audio_ctx.sample_rate as f64) as usize
    }
}
