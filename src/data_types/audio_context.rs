#[derive(Clone, Default)]
pub struct AudioContext {
    pub channels: usize,
    pub sample_rate: usize,
    pub buffer_size: usize,
    pub max_voices: usize,
}
