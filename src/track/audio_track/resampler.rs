pub fn resample_channels(
    source: &[f32],
    source_frames: usize,
    source_sample_rate: usize,
    source_channels: usize,
    target_sample_rate: usize,
    target_channels: usize,
) -> Vec<f32> {
    // Calculate the ratio of the source and the target sample rate
    let ratio = source_sample_rate as f32 / target_sample_rate as f32;
    let mut read_pos = 0.0;
    let mut output = Vec::new();

    while read_pos + 1.0 < source_frames as f32 {
        // Calculate the index from the read position
        let index = read_pos.floor() as usize;
        let remainder = read_pos - read_pos.floor();

        for target_channel in 0..target_channels {
            // Complement the sample in the index, or push zero
            if target_channel < source_channels {
                // Get the two samples to interpolate the sample
                let src_before = source[index * source_channels + target_channel];
                let src_after = source[(index + 1) * source_channels + target_channel];
                // LERP
                output.push(src_before * (1.0 - remainder) + src_after * remainder);
            } else {
                // Push zero if the channel doesn't exist in the source buffer
                output.push(0.0);
            }
        }

        read_pos += ratio;
    }

    output
}
