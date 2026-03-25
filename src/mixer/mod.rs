mod project;
mod tempo_event;
mod tempo_map;
mod track_id;

pub use project::Project;
pub use tempo_event::TempoEvent;
pub use tempo_map::TempoMap;

use crate::{data_types::Beats, graph::error::GraphError};

pub struct Mixer {
    // --- PROJECT ---
    pub project: Project,
}

impl Mixer {
    // --- NEW ---

    /// Creates a new mixer instance with the given project.
    pub fn new(project: Project) -> Self {
        Self { project }
    }

    // --- PROJECT APPLYING ---

    /// Replaces the project with the new one. Tracks inside the project must have been prepared.
    pub fn apply_project(&mut self, new_project: Project) {
        self.project = new_project;
    }

    // --- SEEKING ---

    /// Tells every tracks that the it will seek.
    pub fn seek(&mut self) {
        for track in self.project.tracks.values_mut() {
            track.seek();
        }
    }

    // --- MIXING PROCESS ---

    /// Prepares the tracks in the mixer for the playback.
    /// `start` and `duration` indicates the range to be processed.
    pub fn prepare(&mut self, start: Beats, duration: Beats) -> Result<(), GraphError> {
        // Convert the start and duration beats to samples
        let start_samples = self.project.tempo_map.beats_to_samples(start);
        let duration_samples = self.project.tempo_map.beats_to_samples(duration);

        // Prepare the tracks one by one
        for track in self.project.tracks.values_mut() {
            track.prepare(start_samples, duration_samples, &self.project.tempo_map)?;
        }

        Ok(())
    }

    /// Processes the tracks in the mixer a the specified playhead.
    pub fn process(&mut self, playhead: usize, output: *mut u8) {
        // Fill the output buffer with zeros before processing
        unsafe {
            let len = self.project.audio_ctx.buffer_size * self.project.audio_ctx.channels;
            let dst = std::slice::from_raw_parts_mut(output as *mut f32, len);
            dst.fill(0.0);
        }

        // Call process function for every tracks
        for track in self.project.tracks.values_mut() {
            track.process(playhead, output);
        }
    }
}
