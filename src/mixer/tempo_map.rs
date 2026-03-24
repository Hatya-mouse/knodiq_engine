use crate::data_types::Beats;

#[derive(Default)]
pub struct TempoMap {
    events: Vec<TempoEvent>,
}

pub struct TempoEvent {
    pub beat: Beats,
    pub bpm: f64,
}

impl TempoMap {
    pub fn new(initial_bpm: f64) -> Self {
        Self {
            events: vec![TempoEvent {
                beat: Beats(0.0),
                bpm: initial_bpm,
            }],
        }
    }

    pub fn beats_to_samples(&self, beats: Beats, sample_rate: usize) -> usize {
        // Take the first bpm for now
        let bpm = self.events[0].bpm;
        println!(
            "bpm: {}, beats: {:?}, sample_rate: {}",
            bpm, beats, sample_rate
        );
        (beats.0 / bpm * 60.0 * sample_rate as f64) as usize
    }
}
