// main.rs
// © 2025 Shuntaro Kasatani

use cpal::traits::HostTrait;

mod audio_engine;

fn main() {
    // Create a new audio player
    let mut player = audio_engine::audio_player::AudioPlayer::new();
    // Load the source from a file path
    let mut source = audio_engine::source::AudioSource::new(
        "C:/Users/shunt/Documents/programs/games/godot/air-international-inc/Assets/Audio/Music/Airborne.wav",
        0,
    ).unwrap();
    // Normalize the audio source
    source.normalize();
    // Resample the audio source
    let device_manager = audio_engine::output_device_manager::OutputDeviceManager::new();
    let device = cpal::default_host().default_output_device().unwrap();
    source = device_manager.process_audio(&device, &source).unwrap();
    // Load the source into the player
    player
        .load_source(source)
        .expect("AudioPlayer loading error");
    // Play the audio source
    player.play(None).expect("AudioPlayer playback error");
    // Wait for the audio to finish playing
    player.wait_for_finish();
    std::thread::sleep(std::time::Duration::from_secs(10));
}
