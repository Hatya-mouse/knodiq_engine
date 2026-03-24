use crate::data_types::Beats;

/// Stores the raw audio source data.
pub struct AudioRegion {
    pub data: Vec<f32>,
    pub frames: usize,
    pub sample_rate: u32,
    pub channels: u16,
    pub base_bpm: f64,
    pub start: Beats,
    pub duration: Beats,
}
