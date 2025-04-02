// main.rs
// © 2025 Shuntaro Kasatani

use std::process::exit;

use crate::mixer::traits::track::Track;

mod audio_engine;
mod mixer;

fn main() {
    let sample_rate = 48000;
    let path = "C:/Users/shunt/Documents/programs/games/godot/air-international-inc/Assets/Audio/Music/Airborne.wav";
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
    track.add_region(region);

    // Set the sample rate and channels
    player.sample_rate = sample_rate;
    player.channels = 2;

    // Render the track
    track.render();

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
