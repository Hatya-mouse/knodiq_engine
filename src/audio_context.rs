#[derive(Clone)]
pub struct AudioContext {
    pub channels: u16,
    pub sample_rate: u32,
    pub buffer_size: u32,
}
