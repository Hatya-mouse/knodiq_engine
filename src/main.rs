mod audio_engine;
mod gui;

use crate::audio_engine::buffer_player::SAudioBufferPlayer;
use gui::file_loader::FileLoader;
use iced::settings::Settings;

fn main() {
    // Show the window.
    iced::application("title", FileLoader::update, FileLoader::view)
        .settings(Settings::default())
        .run()
        .unwrap();
}
