use crate::audio_engine::audio_buffer::SAudioBuffer;
use crate::audio_engine::buffer_player::SAudioBufferPlayer;
use std::sync::{mpsc, Arc};
use std::thread;

pub enum AudioCommand {
    Play,
    Pause,
    Stop,
}

pub struct AudioPlayerController {
    sender: mpsc::Sender<AudioCommand>,
    handle: Option<thread::JoinHandle<()>>,
}

impl AudioPlayerController {
    pub fn new(buffer: SAudioBuffer) -> Self {
        let (tx, rx) = mpsc::channel();
        let buffer = Arc::new(buffer);
        let handle = Some(Self::spawn_audio_thread(rx, Arc::clone(&buffer)));

        Self { sender: tx, handle }
    }

    pub fn set_buffer(&mut self, buffer: SAudioBuffer) {
        let (tx, rx) = mpsc::channel();
        self.sender = tx;
        let buffer = Arc::new(buffer);
        self.sender.send(AudioCommand::Stop).unwrap();
        self.handle = Some(Self::spawn_audio_thread(rx, Arc::clone(&buffer)));
    }

    fn spawn_audio_thread(
        rx: mpsc::Receiver<AudioCommand>,
        buffer: Arc<SAudioBuffer>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut player = SAudioBufferPlayer::new();
            player.set_buffer(Arc::clone(&buffer));
            loop {
                match rx.recv() {
                    Ok(AudioCommand::Play) => {
                        if let Err(e) = player.play() {
                            eprintln!("Error while playing: {}", e);
                        }
                    }
                    Ok(AudioCommand::Pause) => {
                        println!("Paused");
                    }
                    Ok(AudioCommand::Stop) | Err(_) => {
                        println!("Stopping playback");
                        break;
                    }
                }
            }
        })
    }

    pub fn send_command(&self, command: AudioCommand) {
        if let Err(e) = self.sender.send(command) {
            eprintln!("Failed to send command: {}", e);
        }
    }
}
