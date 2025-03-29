use iced::widget::{button, column, text};
use iced::Element;
use rfd::FileDialog;
use std::process::Command;

use crate::audio_engine::audio_buffer::SAudioBuffer;
use crate::audio_engine::audio_player_controller::AudioCommand;
use crate::audio_engine::audio_player_controller::AudioPlayerController;
use crate::SAudioBufferPlayer;

#[derive(Debug, Clone)]
enum Message {
    OpenFileDialog,
    Play,
}

#[derive(Default)]
pub struct FileLoader {
    file_path: Option<String>,
    player_controller: Option<AudioPlayerController>,
}

impl FileLoader {
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let player_controller = AudioPlayerController::new();

        (
            Self {
                player_controller: Some(player_controller),
            },
            Command::none(),
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenFileDialog => {
                // Show the FileDialog and pick a file.
                let path_buf = FileDialog::new()
                    .set_title("Select an Audio File")
                    .add_filter("Audio Files", &["mp3", "wav", "flac"])
                    .pick_file();
                let file_path = match path_buf {
                    Some(path) => match path.to_str() {
                        Some(path) => path.to_owned(),
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                };

                self.file_path = Some(file_path);
            }
            Message::Play => {
                if let Some(controller) = &self.player_controller {
                    let track_number: usize = 0;
                    let mut audio_buffer =
                        SAudioBuffer::new(self.file_path.as_ref().unwrap(), track_number)
                            .expect("Failed to initialize the audio buffer");
                    audio_buffer.normalize();
                    controller.send_command(AudioCommand::Play)
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            button("Select File").on_press(Message::OpenFileDialog),
            button("Play").on_press(Message::Play),
            text(self.file_path.as_deref().unwrap_or("No file selected"),)
        ]
        .into()
    }
}
