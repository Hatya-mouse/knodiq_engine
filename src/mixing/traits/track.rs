// track.rs
// Trait that represents a track.
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

use crate::Region;
use crate::audio_context::AudioContext;
use crate::audio_utils::Beats;
use crate::error::track::TrackError;
use crate::mixing::region::RegionID;

pub trait Track {
    /// Adds a new region to the track.
    /// # Arguments
    /// - `region` - The region to add.
    /// - `at` - The position in beats where the region should be added.
    /// - `duration` - The duration of the region in beats.
    ///
    /// # Return
    /// - The unique identifier of the added region.
    fn add_region(
        &mut self,
        region: Box<dyn Region>,
        at: Beats,
        duration: Beats,
    ) -> Result<RegionID, TrackError>;

    /// Removes the specified region from the track.
    fn remove_region(&mut self, id: &RegionID);

    /// Returns the type of the track in the form of a string.
    fn track_type(&self) -> String;

    /// Returns the duration of the track in beats.
    fn duration(&self) -> Beats;

    /// Prepare the track for rendering.
    fn prepare(&mut self, audio_ctx: &AudioContext) -> Result<(), TrackError>;

    /// Renders the specified area of the track.
    ///
    /// # Arguments
    /// - `audio_ctx` - The current audio context.
    fn process(&mut self, playhead: Beats, audio_ctx: &AudioContext);
}
