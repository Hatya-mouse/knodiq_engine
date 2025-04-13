// audio_buffer.rs
// Represents an audio buffer in the audio source.
// © 2025 Shuntaro Kasatani

use crate::buffer::sample::Sample;

pub type AudioBuffer = Vec<Vec<Sample>>;
