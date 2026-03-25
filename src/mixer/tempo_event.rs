use crate::data_types::Beats;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct TempoEvent {
    pub beat: Beats,
    pub bpm: f64,
    pub sample_offset: usize,
}

impl PartialEq for TempoEvent {
    fn eq(&self, other: &Self) -> bool {
        self.beat == other.beat
    }
}

impl Eq for TempoEvent {}

impl PartialOrd for TempoEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TempoEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.beat > other.beat {
            Ordering::Greater
        } else if self.beat == other.beat {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
