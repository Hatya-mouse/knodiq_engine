use crate::data_types::Beats;
use std::cmp::Ordering;

pub struct TempoEvent {
    pub beat: Beats,
    pub bpm: f64,
    pub sample_offset: usize,
}

impl PartialEq for TempoEvent {
    fn eq(&self, other: &Self) -> bool {
        self.beat == other.beat
    }

    fn ne(&self, other: &Self) -> bool {
        self.beat != other.beat
    }
}

impl Eq for TempoEvent {}

impl PartialOrd for TempoEvent {
    fn ge(&self, other: &Self) -> bool {
        self.beat >= other.beat
    }

    fn gt(&self, other: &Self) -> bool {
        self.beat > other.beat
    }

    fn le(&self, other: &Self) -> bool {
        self.beat <= other.beat
    }

    fn lt(&self, other: &Self) -> bool {
        self.beat < other.beat
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(if self.beat > other.beat {
            Ordering::Greater
        } else if self.beat == other.beat {
            Ordering::Equal
        } else {
            Ordering::Less
        })
    }
}

impl Ord for TempoEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
