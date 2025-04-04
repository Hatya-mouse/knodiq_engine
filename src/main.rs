// main.rs
// © 2025 Shuntaro Kasatani

mod audio_engine;

use std::process::exit;

use audio_engine::mixing::{region::BufferRegion, track::BufferTrack};
use audio_engine::{AudioPlayer, AudioSource, Mixer};

fn main() {
    let sample_rate = 48000;
    let path1 = "/Users/shuntaro/Music/Music/Media.localized/Music/ShinkoNet/Hypixel Skyblock Original Sound Track/3-02 Superior Judgement.mp3";
    let path2 = "/Users/shuntaro/Music/Music/Media.localized/Music/ShinkoNet/Hypixel Skyblock Original Sound Track/3-03 Dungeon Drama.mp3";

    // Load the source 1 from a file path
    let mut source1 = AudioSource::from_path(path1, 0).unwrap();
    // Normalize the audio source 1
    source1.normalize();

    // Load the source 2 from a file path
    let mut source2 = AudioSource::from_path(path2, 0).unwrap();
    // Normalize the audio source 2
    source2.normalize();

    // Create a region
    let region1 = BufferRegion::new(source1);
    // Create a track
    let mut track1 = BufferTrack::new(0, "Track 1", 2);
    // Add a region to the track
    track1.add_region(region1);
    track1.graph.connect(
        track1.graph.input_nodes[0],
        "output".to_string(),
        track1.graph.output_node,
        "input".to_string(),
    );

    // Create a region
    let region2 = BufferRegion::new(source2);
    // Create a track
    let mut track2 = BufferTrack::new(0, "Track 2", 2);
    // Add a region to the track
    track2.add_region(region2);
    track2.graph.connect(
        track2.graph.input_nodes[0],
        "output".to_string(),
        track2.graph.output_node,
        "input".to_string(),
    );

    // Create a new audio player
    let mut player = AudioPlayer::new();

    // Set the sample rate and channels
    let sample_sender = player
        .initialize_player(sample_rate, 2)
        .expect("Playback error");

    // Create a mixer
    let mut mixer = Mixer::new(sample_rate, 2);
    mixer.add_track(Box::new(track1));
    mixer.add_track(Box::new(track2));

    // Move sample_sender into the closure to fix lifetime issues
    let rendered_data = {
        let sender = sample_sender;
        mixer.mix(Box::new(move |sample| {
            let _ = sender.send(sample);
        }))
    };
    println!("Rendered data sample rate: {}", rendered_data.sample_rate);

    player.completion_handler = Some(Box::new(|| {
        exit(0);
    }));

    loop {
        player.update();
    }
}
