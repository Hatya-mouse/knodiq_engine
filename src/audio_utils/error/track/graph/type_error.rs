// type_error.rs
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

use crate::{Type, error::TrackError};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct TypeError {
    pub expected_type: Type,
    pub received_type: Type,
}

impl TrackError for TypeError {
    fn clone_box(&self) -> Box<dyn TrackError> {
        Box::new(self.clone())
    }
}

impl std::error::Error for TypeError {}

impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Expected type '{}', but received type '{}'",
            self.expected_type, self.received_type
        )
    }
}
