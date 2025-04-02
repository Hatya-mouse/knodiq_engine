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
