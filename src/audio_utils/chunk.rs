// chunk.rs
// Separates audio buffer into smaller chunks of a given size.
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

use std::cmp;

/// Chunk buffer into smaller chunks of a given size.
/// Last chunk may be smaller than chunk_size.
///
/// Arguments:
/// * `buffer`: The buffer to be chunked. Outer vector represents channels, inner vector represents samples.
/// * `chunk_size`: The size of each chunk.
pub fn chunk_buffer(buffer: &Vec<Vec<f32>>, chunk_size: usize) -> Vec<Vec<Vec<f32>>> {
    let mut chunks: Vec<Vec<Vec<f32>>> = Vec::new();
    let mut current_index = 0;

    while current_index < buffer[0].len() - 1 {
        // Get chunk from the buffer
        let mut chunk = Vec::new();
        let end_index = cmp::min(current_index + chunk_size, buffer[0].len() - 1);
        for channel in buffer {
            let channel_chunk = channel[current_index..end_index].to_vec();
            chunk.push(channel_chunk);
        }

        // Push the chunk to the chunks vector
        chunks.push(chunk);
        // Update the current index to the end index of the chunk
        current_index = end_index;
    }

    chunks
}
