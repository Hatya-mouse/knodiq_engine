use crate::audio_engine::node::graph::Graph;
use crate::audio_engine::{chunk, source::AudioSource};
use crate::mixer::region::buffer_region::BufferRegion;
use crate::mixer::traits::{region::Region, track::Track};

pub struct BufferTrack {
    /// Unique identifier for the track.
    pub id: u32,
    /// Name of the track.
    pub name: String,
    /// Volume of the track.
    pub volume: f32,
    /// Audio node graph.
    pub graph: Graph,
    /// Sample rate of the track.
    pub sample_rate: usize,
    /// Number of channels in the track.
    pub channels: usize,
    /// Vector of regions in the track.
    pub regions: Vec<BufferRegion>,
    /// Rendered audio data.
    pub rendered_data: Option<AudioSource>,
}

impl BufferTrack {
    pub fn new(id: u32, name: &str, sample_rate: usize, channels: usize) -> Self {
        Self {
            id,
            name: name.to_string(),
            volume: 1.0,
            graph: Graph::new(),
            sample_rate,
            channels,
            regions: Vec::new(),
            rendered_data: None,
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

    fn volume(&self) -> f32 {
        self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    fn sample_rate(&self) -> usize {
        self.sample_rate
    }

    fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate;
    }

    fn render(&mut self) {
        // Create a new audio source with the same sample rate and channels
        let mut output_audio_source = AudioSource::new(self.sample_rate, self.channels);

        for region in &self.regions {
            // Get the data from the audio source
            let owned_data = region.audio_source().data.clone();
            // Split the data to multiple chunks
            let chunks = chunk::chunk_buffer(&owned_data, self.sample_rate);

            let mut progress_counter = 0;
            let total_chunks = chunks.len();
            // Loop through each chunk
            for chunk in chunks {
                // Process the chunk
                let processed_chunk = match self.graph.process(AudioSource {
                    data: chunk,
                    sample_rate: self.sample_rate,
                    channels: self.channels,
                }) {
                    Ok(chunk) => chunk,
                    Err(err) => {
                        eprintln!("Error processing chunk: {}", err);
                        continue;
                    }
                };

                // Add the chunk to the audio source
                for (i, channel) in output_audio_source.data.iter_mut().enumerate() {
                    if i < processed_chunk.channels {
                        channel.extend(processed_chunk.data[i].to_owned());
                    }
                }

                progress_counter += 1;

                println!(
                    "Processing chunk... {}%",
                    progress_counter as f32 / total_chunks as f32 * 100.0
                );
            }
        }

        self.rendered_data = Some(output_audio_source);
    }

    fn rendered_data(&self) -> Result<&AudioSource, Box<dyn std::error::Error>> {
        match self.rendered_data {
            Some(ref data) => Ok(data),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No rendered data available",
            ))),
        }
    }
}
