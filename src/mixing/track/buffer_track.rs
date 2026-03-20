// buffer_track.rs
// A type of buffer that stores buffer as audio data.
//
// Copyright 2025 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use symphonia::core::audio;

use crate::{
    AudioResampler, AudioSource, Graph, Region, Track, Value,
    audio_context::AudioContext,
    audio_utils::{self, Beats},
    error::track::{TrackError, TrackErrorKind},
    graph::built_in::InputNode,
    mixing::region::{BufferRegion, RegionID},
};
use std::collections::HashMap;

pub struct BufferTrack {
    /// Audio node graph.
    pub graph: Graph,
    /// Vector of regions in the track.
    pub regions: HashMap<RegionID, BufferRegion>,
    /// Rendered audio data.
    pub rendered_data: Option<AudioSource>,
    /// Resampled region sources.
    resampled: HashMap<RegionID, AudioSource>,
    /// Error that occurred during processing.
    process_error: Option<TrackError>,
    /// Original mixed data before processing.
    original_mixed_data: Option<AudioSource>,
    /// Next region id.
    next_region_id: usize,

    /// The start sample index of the regions.
    pub region_start_indices: HashMap<RegionID, usize>,
}

impl BufferTrack {
    pub fn new(name: &str, channels: usize) -> Self {
        Self {
            graph: Graph::new(Box::new(InputNode::default())),
            regions: HashMap::new(),
            rendered_data: None,
            resampled: HashMap::new(),
            process_error: None,
            original_mixed_data: None,
            next_region_id: 0,
            region_start_indices: HashMap::new(),
        }
    }

    fn generate_region_id(&mut self) -> RegionID {
        let id = RegionID::new(self.next_region_id);
        self.next_region_id += 1;
        id
    }
}

impl Track for BufferTrack {
    fn track_type(&self) -> String {
        "BufferTrack".to_string()
    }

    fn add_region(
        &mut self,
        region: Box<dyn Region>,
        at: Beats,
        duration: Beats,
    ) -> Result<RegionID, TrackError> {
        if let Some(buffer_region) = region.as_any().downcast_ref::<BufferRegion>() {
            let mut buffer_region = buffer_region.clone();
            let id = self.generate_region_id();
            buffer_region.set_start_time(at);
            buffer_region.set_duration(duration);
            self.regions.insert(id, buffer_region);
            Ok(id)
        } else {
            Err(TrackError::new(TrackErrorKind::InvalidRegionTypeError {
                expected_type: "BufferRegion".to_string(),
                received_type: region.region_type(),
            }))
        }
    }

    fn remove_region(&mut self, id: &RegionID) {
        self.regions.remove(id);
    }

    fn duration(&self) -> Beats {
        self.regions
            .iter()
            .map(|r| r.end_time())
            .fold(0.0, |max, end| max.max(end))
    }

    fn prepare(&mut self, audio_ctx: &AudioContext) -> Result<(), TrackError> {
        // Create a resampler and resample all regions
        for (region_id, region) in self.regions.iter() {
            // Resample the regions
            let resampler = AudioResampler::new(
                audio_ctx.buffer_samples,
                audio_ctx.channels,
                region.sample_rate,
                audio_ctx.sample_rate,
            )
            .map_err(|e| TrackError::new(TrackErrorKind::ResamplerConstructionError(e)))?;
            let resampled = region
                .audio_source()
                .map(|src| resampler.process(region.src));

            // Calculate the start index of the regions
            let region_start_index =
                audio_utils::beats_as_samples(audio_ctx.samples_per_beat, region.start_time);
            self.region_start_indices
                .insert(*region_id, region_start_index);
        }

        // Prepare the graph
        self.graph.prepare(audio_ctx)?;

        Ok(())
    }

    fn process(&mut self, playhead: Beats, audio_ctx: &AudioContext) {
        self.render_error = None;

        // Mixed audio data for the chunk
        let mut mixed = AudioSource::zeros(
            audio_ctx.sample_rate,
            self.channels,
            audio_ctx.buffer_samples,
        );

        // Convert the playhead beats to sample index
        let playhead_index = audio_utils::beats_as_samples(audio_ctx.samples_per_beat, playhead);

        for (region_id, region) in self
            .regions
            .iter_mut()
            .filter(|(_, r)| r.is_active_at(playhead, playhead + audio_ctx.chunk_size))
        {
            // If the region does not have an audio source, skip it
            if region.audio_source().is_none() {
                continue;
            }

            // MIX THE REGION TO THE MIXED AUDIO SOURCE
            // 1. Get a relative playhead position in the region
            let rel_playhead = playhead_index - self.region_start_indices[region_id];

            // 2. Get a reference to the region

            // Mix the sliced chunk into the mixed audio data
            mixed.mix_at(&resampled_region, 0);
        }

        match self.original_mixed_data.as_mut() {
            Some(original) => original.mix_at(&mixed, playhead_samples),
            None => self.original_mixed_data = Some(mixed),
        }

        let original_mixed_data = self.original_mixed_data.as_ref().expect("IMPOSSIBLE");

        // Pass the resampled chunk to the graph input node
        if let Some(input_node) = self.graph.get_input_node_mut() {
            input_node.set_input(
                "audio",
                Value::from_buffer(original_mixed_data.data.clone()),
            );
        }

        // Process the chunk through the graph
        let mut processed = match self.graph.process(audio_ctx) {
            Ok(chunk) => chunk,
            Err(err) => {
                self.render_error = Some(err);
                AudioSource::zeros(
                    audio_ctx.sample_rate,
                    self.channels,
                    playhead_samples + audio_ctx.buffer_samples,
                )
            }
        };

        if processed.samples() > audio_ctx.buffer_samples {
            processed.slice(0, audio_ctx.buffer_samples);
        } else if processed.samples() < audio_ctx.buffer_samples {
            // If the mixed data is shorter than the chunk size, pad it with zeros
            processed.pad(audio_ctx.buffer_samples - processed.samples());
        }

        self.rendered_data = Some(processed);
    }
}
