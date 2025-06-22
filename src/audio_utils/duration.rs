// duration.rs
// Converts between std::time::Duration and sample count
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

pub type Beats = f32;

pub fn samples_as_beats(samples_per_beat: Beats, samples: usize) -> Beats {
    samples as Beats / samples_per_beat
}

pub fn beats_as_samples(samples_per_beat: Beats, beats: Beats) -> usize {
    (beats * samples_per_beat).round() as usize
}
