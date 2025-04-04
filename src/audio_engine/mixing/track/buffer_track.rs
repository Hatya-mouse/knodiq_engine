// buffer_track.rs
// A type of buffer that stores buffer as audio data.
// © 2025 Shuntaro Kasatani

use std::ops::DerefMut;

use crate::audio_engine::{
    mixing::region::BufferRegion, utils::ansi, utils::chunk, AudioResampler, AudioSource, Graph,
    Region, Track,
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

    fn render(&mut self, sample_rate: usize, callback: &mut Box<dyn FnMut(f32)>) {
        // Define the chunk size for processing
        let chunk_size = 1024;

        // Create a new audio source with the same sample rate and channels
        let mut output_audio_source = AudioSource::new(sample_rate, self.channels);

        // Print that the track is being processed
        println!(
            "{}{}Processing the track {}\"{}\"{}",
            ansi::BOLD,
            ansi::GREEN,
            ansi::WHITE,
            self.name(),
            ansi::RESET,
        );

        for (region_index, region) in self.regions.iter().enumerate() {
            // Get the audio source from the region
            let owned_data = region.audio_source().clone();
            // Split the data into chunks
            let chunks = chunk::chunk_buffer(&owned_data.data, chunk_size);

            // Create a new audio resampler to resample the audio region
            let mut resampler = AudioResampler::new(chunk_size);

            // Prepare the graph for processing
            self.graph.prepare(chunk_size);

            // Get the chunk number to calculate the progress
            let total_chunks = chunks.len();
            // Width of the progress bar
            let bar_width = 40;

            println!(
                "{}{}Processing region{} {}{}",
                ansi::BOLD,
                ansi::CYAN,
                ansi::WHITE,
                region_index,
                ansi::RESET
            );

            // Loop through each chunk
            for (chunk_index, chunk) in chunks.iter().enumerate() {
                // Process the chunk
                let processed_chunk = match self.graph.process(AudioSource {
                    data: chunk.clone(),
                    sample_rate: owned_data.sample_rate,
                    channels: owned_data.channels,
                }) {
                    Ok(chunk) => chunk,
                    Err(err) => {
                        eprintln!("Error processing chunk: {}", err);
                        continue;
                    }
                };

                // Resample the processed region
                let resampled_audio_source = match resampler.process(processed_chunk, sample_rate) {
                    Ok(resampled) => resampled,
                    Err(err) => {
                        // Is the resample has failed, print the error message and return None
                        eprintln!("Error resampling audio: {}", err);
                        AudioSource::new(0, 0)
                    }
                };

                // Call the callback function with the resampled audio data
                println!(
                    "Channel count is {}",
                    resampled_audio_source.samples() == resampled_audio_source.data[1].len()
                );
                for sample_index in 0..resampled_audio_source.samples() {
                    for channel_index in 0..resampled_audio_source.channels {
                        let sample = resampled_audio_source.data[channel_index][sample_index];
                        // Call the callback function with the sample
                        callback.as_mut()(sample);
                    }
                }

                // add the chunk to the audio source
                for (i, channel) in output_audio_source.data.iter_mut().enumerate() {
                    if i < resampled_audio_source.channels {
                        channel.extend(resampled_audio_source.data[i].to_owned());
                    }
                }

                // Print the progress bar
                let percentage = (chunk_index as f32 / total_chunks as f32) * 100.0;
                let filled = (percentage as usize * bar_width) / 100;
                print!(
                    "\r[{}{}{}] {}{:.1}%{} ({}/{}) ",
                    "=".repeat(filled),
                    ">",
                    " ".repeat(bar_width - filled),
                    ansi::BOLD,
                    percentage,
                    ansi::RESET,
                    chunk_index + 1,
                    total_chunks
                );

                // Flush stdout to ensure the line is displayed
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
            }
            // Print the new line
            println!();
        }

        // Set the rendered data
        self.rendered_data = Some(output_audio_source);
    }

    fn rendered_data(&self) -> Result<&AudioSource, Box<dyn std::error::Error>> {
        match self.rendered_data {
            Some(ref data) => Ok(data),
            None => Err("No rendered data available".into()),
        }
    }
}
