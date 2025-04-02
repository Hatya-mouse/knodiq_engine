// main.rs
// © 2025 Shuntaro Kasatani

use std::process::exit;

use crate::audio_engine::node::built_in::resample::AudioResampler;
use crate::mixer::traits::track::Track;

mod audio_engine;
mod mixer;

fn main() {
    let sample_rate = 48000;
    let path = "/Users/shuntaro/Music/Music/Media.localized/Music/ShinkoNet/Hypixel Skyblock Original Sound Track/1-05 Let Them Eat Cake.mp3";

    // Create a new audio player
    let mut player = audio_engine::audio_player::AudioPlayer::new();
    // Load the source from a file path
    let mut source = audio_engine::source::AudioSource::from_path(path, 0).unwrap();
    // Normalize the audio source
    source.normalize();

    // Create a region
    let region = mixer::region::buffer_region::BufferRegion::new(source);
    // Create a track
    let mut track = mixer::track::buffer_track::BufferTrack::new(0, "Track 0", sample_rate, 2);
    // Add a region to the track
    track.add_region(region);
    // Add a resampler to the track
    let resampler_node = AudioResampler::new(sample_rate);
    let resampler_node_id = track.graph.add_node(Box::new(resampler_node));
    track.graph.connect(
        track.graph.input_nodes[0],
        "output".to_string(),
        resampler_node_id,
        "input".to_string(),
    );
    track.graph.connect(
        resampler_node_id,
        "output".to_string(),
        track.graph.output_node,
        "input".to_string(),
    );

    track.sample_rate = sample_rate;

    // Render the track
    track.render();

    // Set the sample rate and channels
    player.channels = 2;
    player.sample_rate = sample_rate;

    player
        .add_queue(track.rendered_data().unwrap().data.clone())
        .expect("Die.");

    player.completion_handler = Some(Box::new(|| {
        exit(0);
    }));

    loop {
        player.update();
    }
}
