mod audio_engine;

fn main() {
    // Create a new audio player
    let mut player = audio_engine::audio_player::AudioPlayer::new().unwrap();
    // Load the source from a file path
    let mut source = audio_engine::source::AudioSource::new(
        r"C:\Users\shunt\Documents\programs\games\godot\air-international-inc\Assets\Audio\Music\Airborne.wav",
        0,
    ).unwrap();
    // Normalize the audio source
    source.normalize();
    // Load the source into the player
    player
        .load_source(source)
        .err()
        .map(|err| println!("{}", err));
    // Play the audio source
    player.play().err().map(|err| println!("{}", err));
    // Wait for the audio to finish playing
    player.wait_for_finish();
}
