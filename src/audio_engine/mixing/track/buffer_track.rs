// buffer_track.rs
// A type of buffer that stores buffer as audio data.
// © 2025 Shuntaro Kasatani

use crate::audio_engine::{
    audio_utils, mixing::region::BufferRegion, AudioResampler, AudioSource, Duration, Graph,
    Region, Track,
};
use crate::utils::ansi;

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
    /// Resamplers for each regions.
    resamplers: Vec<AudioResampler>,
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
            resamplers: Vec::new(),
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

    fn prepare(&mut self, _sample_rate: usize) {
        let chunk_size = 1024;
        self.graph.prepare(chunk_size);
        self.resamplers
            .resize_with(self.regions.len(), || AudioResampler::new(chunk_size));
    }

    fn render_chunk_at(
        &mut self,
        playhead: Duration,
        chunk_size: Duration,
        sample_rate: usize,
    ) -> bool {
        // Clear the rendered data
        self.rendered_data = Some(AudioSource::new(sample_rate, self.channels));

        // Whether the rendering has finished
        let mut completed = true;

        for (region_index, region) in self
            .regions
            .iter_mut()
            .filter(|r| r.is_active_at(playhead, playhead + chunk_size))
            .enumerate()
        {
            if playhead < region.end_time() {
                completed = false;
            }

            let region_source = region.audio_source();

            // Calculate the area to be sliced
            // ———————————————————————————————
            // Start sample index of the region (in Region sample rate) (in global position)
            let region_start =
                audio_utils::as_samples(region_source.sample_rate, region.start_time());
            // Playhead position (in Region sample rate)
            let region_playhead = audio_utils::as_samples(region_source.sample_rate, playhead);
            // Chunk size (in Region sample rate)
            let region_chunk_size = audio_utils::as_samples(region_source.sample_rate, chunk_size);

            // Calculate the range to slice (in Region sample rate) (in Region-based position)
            let start_sample = region_playhead.saturating_sub(region_start);
            //  |    |    | [ R>E G |I O |N ] |    |    |    |    |    |    |
            // region_start ^  ^    ^ region_playhead + region_chunk_size
            //
            // >: Playhead, |: Chunk separation
            let end_sample = (start_sample + region_chunk_size).clamp(0, region_source.samples());

            // Slice the region to get the chunk
            let mut chunk = AudioSource::new(region_source.sample_rate, self.channels);
            for ch in 0..self.channels {
                chunk.data[ch].extend_from_slice(&region_source.data[ch][start_sample..end_sample]);
            }

            // Resample the chunk with the resampler dedicated to the region
            let resampled = match self.resamplers[region_index].process(chunk, sample_rate) {
                Ok(chunk) => chunk,
                Err(_) => continue,
            };

            // Process the chunk through the graph
            let processed = match self.graph.process(resampled) {
                Ok(chunk) => chunk,
                Err(_) => continue,
            };

            if let Some(ref mut data) = self.rendered_data {
                // Calculate the chunk start position (in chunk-based position)
                let region_start_in_chunk = region.start_time().saturating_sub(playhead);
                // Add the processed chunk to the rendered data at the chunk start position
                data.mix_at(&processed, region_start_in_chunk);
            }
        }

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
