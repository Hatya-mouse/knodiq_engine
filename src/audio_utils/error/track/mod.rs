// audio_utils/error/track/mod.rs
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

pub mod graph;
pub mod region;
pub mod unknown_track_error;

pub use graph::{
    node_cycle_error::NodeCycleError, node_input_type_error::NodeInputTypeError,
    node_not_found_error::NodeNotFoundError, node_output_type_error::NodeOutputTypeError,
    property_not_found_error::PropertyNotFoundError, type_error::TypeError,
};
pub use region::InvalidRegionTypeError;
pub use unknown_track_error::UnknownTrackError;

use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub trait TrackError: Error + Debug + Display + Send + Sync + 'static {
    fn clone_box(&self) -> Box<dyn TrackError>;
}

// Implement Clone for Box<dyn TrackError>
impl Clone for Box<dyn TrackError> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
