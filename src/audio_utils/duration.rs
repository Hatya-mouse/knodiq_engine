// duration.rs
// Converts between std::time::Duration and sample count
// © 2025 Shuntaro Kasatani

pub type Beats = f32;

pub fn samples_as_beats(samples_per_beat: Beats, samples: usize) -> Beats {
    samples as Beats / samples_per_beat
}

pub fn beats_as_samples(samples_per_beat: Beats, beats: Beats) -> usize {
    (beats * samples_per_beat).round() as usize
}
