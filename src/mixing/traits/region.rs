// region.rs
// A trait that represents a region in the track.
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

use crate::audio_utils::Beats;
use std::any::Any;

pub trait Region: Send + Sync + Any {
    /// Returns the id of the region.
    fn get_id(&self) -> &u32;

    /// Sets the id of the region.
    fn set_id(&mut self, id: u32);

    /// Returns the start time of the region in f32.
    /// Sets the name of the region.
    fn set_name(&mut self, name: String);

    /// Returns the name of the region.
    fn get_name(&self) -> &str;

    /// Read the name and you'll know what this does.
    fn start_time(&self) -> Beats;

    /// Sets the start time of the region in beats.
    fn set_start_time(&mut self, start_time: Beats);

    /// Sets the duration of the region in beats.
    fn set_duration(&mut self, duration: Beats);

    /// Returns the end time of the region in beats.
    fn end_time(&self) -> Beats;

    /// Returns the duration of the region in beats.
    fn duration(&self) -> Beats;

    /// Scale the region to a new duration in beats.
    fn scale(&mut self, duration: Beats);

    /// Returns whether the region overlaps with the given area.
    fn is_active_at(&self, start: Beats, end: Beats) -> bool {
        // Check if the chunk overlaps with the region.
        (start >= self.start_time() && start <= self.end_time())
            || (end >= self.start_time() && end <= self.end_time())
    }

    /// Returns the type of the region as a string.
    fn region_type(&self) -> String;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
