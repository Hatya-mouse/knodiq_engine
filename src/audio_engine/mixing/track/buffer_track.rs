// buffer_track.rs
// A type of buffer that stores buffer as audio data.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::{
    mixing::region::BufferRegion, AudioResampler, AudioSource, Duration, Graph, Region, Track,
};

pub struct BufferTrack {
    /// Unique identifier for the track.
    pub id: u32,
    /// Name of the track.
    pub name: String,
    /// Volume of the track.
    pub volume: f32,
    /// Audio node graph.
    pub graph: Graph,
    /// Number of channels in the track.
    pub channels: usize,
    /// Vector of regions in the track.
    pub regions: Vec<BufferRegion>,
    /// Rendered audio data.
    pub rendered_data: Option<AudioSource>,
    /// Resampler to resample the buffer.
    resampler: Option<AudioResampler>,
}

impl BufferTrack {
    pub fn new(id: u32, name: &str, channels: usize) -> Self {
        Self {
            id,
            name: name.to_string(),
            volume: 1.0,
            graph: Graph::new(),
            channels,
            regions: Vec::new(),
            rendered_data: None,
            resampler: None,
        }
    }

    pub fn add_region(&mut self, region: BufferRegion) {
        self.regions.push(region);
    }
}

impl Track for BufferTrack {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn graph(&mut self) -> &mut Graph {
        &mut self.graph
    }

    fn volume(&self) -> f32 {
        self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    fn prepare(&mut self, chunk_size: usize) {
        self.graph.prepare(chunk_size);
        self.resampler = Some(AudioResampler::new(chunk_size));
    }

    fn render_chunk_at(
        &mut self,
        playhead: Duration,
        chunk_size: usize,
        sample_rate: usize,
    ) -> bool {
        // Create a new audio source with the same sample rate and channels
        let mut output = AudioSource::new(sample_rate, self.channels);

        // Whether the rendering has finished
        let mut completed = true;

        for region in self
            .regions
            .iter()
            .filter(|r| r.is_active_at(playhead, chunk_size, sample_rate))
        {
            if playhead < region.end_time() {
                completed = false;
            }

            if !region.is_active_at(playhead, chunk_size, sample_rate) {
                continue;
            }

            let source = region.audio_source();
            let region_start = region.start_time();
            let offset = if playhead > region_start {
                playhead - region_start
            } else {
                Duration::ZERO
            };

            let start_sample = (offset.as_secs_f64() * source.sample_rate as f64) as usize;
            let end_sample = (start_sample + chunk_size).min(source.samples());

            let mut chunk = AudioSource::new(sample_rate, self.channels);
            for ch in 0..self.channels {
                chunk.data[ch].extend_from_slice(&source.data[ch][start_sample..end_sample]);
            }

            // Process the chunk through the graph
            let processed = match self.graph.process(chunk) {
                Ok(chunk) => chunk,
                Err(_) => continue,
            };

            // Resample the processed data
            let resampled = match self.resampler {
                Some(ref mut resampler) => match resampler.process(processed, sample_rate) {
                    Ok(chunk) => chunk,
                    Err(_) => continue,
                },
                None => continue,
            };

            output.mix(&resampled);
        }

        // Set the rendered data
        self.rendered_data = Some(output);

        // Return whether the rendering has ended
        completed
    }

    fn rendered_data(&self) -> Result<&AudioSource, Box<dyn std::error::Error>> {
        match self.rendered_data {
            Some(ref data) => Ok(data),
            None => Err("No rendered data available".into()),
        }
    }
}
