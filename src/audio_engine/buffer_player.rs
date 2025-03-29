use crate::audio_engine::audio_buffer::SAudioBuffer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleRate;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

pub enum AudioCommand {
    Play,
    Pause,
    Stop,
}

pub struct SAudioBufferPlayer {
    pub buffer: Option<Arc<SAudioBuffer>>,
    stream: Option<cpal::Stream>,
    sender: mpsc::Sender<AudioCommand>,
    receiver: mpsc::Receiver<AudioCommand>,
    handle: Option<thread::JoinHandle<()>>,
}

impl SAudioBufferPlayer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            buffer: None,
            stream: None,
            sender: tx,
            receiver: rx,
            handle: None,
        }
    }

    pub fn set_buffer(&mut self, buffer: Arc<SAudioBuffer>) {
        self.buffer = Some(buffer);
    }

    pub fn play(&self) -> Result<(), Box<dyn std::error::Error>> {
        let buffer = match self.buffer.as_ref() {
            Some(b) => b,
            None => return Err("No buffer set".into()),
        };

        let volume_factor = 1.0;

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or("No default output device")?;

        // Get the output config from the output device
        let config = device.default_output_config()?;
        let mut stream_config = config.config();

        stream_config.sample_rate = SampleRate(buffer.sample_rate);
        println!("Sample rate: {}", buffer.sample_rate);

        // Get the number of channels in the buffer
        let channels = buffer.channels();
        let frames = buffer.samples();

        let owned_data = buffer.data.to_owned();

        // Spawn a thread to wait for the playback to finish
        let handle = thread::spawn(move || -> Result<(), Box<dyn std::error::Error>> {
            let (tx, rx) = mpsc::channel();

            let mut frame_index = 0;

            // Build the output stream...
            let stream = device.build_output_stream(
                &stream_config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for frame_slice in data.chunks_mut(channels) {
                        let mut channel_index = 0;
                        for sample in frame_slice.iter_mut() {
                            if frame_index < frames {
                                *sample = owned_data[channel_index][frame_index] * volume_factor;
                            } else {
                                *sample = 0.0;
                                let _ = tx.send(());
                            }
                            channel_index += 1;
                        }
                        frame_index += 1;
                    }
                },
                move |err| eprintln!("Error occurred during building playback stream: {:?}", err),
                Some(Duration::from_secs(10)),
            )?;

            // ...and play the stream!
            stream.play()?;

            while rx.try_recv().is_err() {
                match receiver.recv() {
                    Ok(AudioCommand::Play) => {
                        if let Err(e) = self.play() {
                            eprintln!("Error while playing: {}", e);
                        }
                    }
                    Ok(AudioCommand::Pause) => {
                        println!("Paused");
                        stream.pause();
                    }
                    Ok(AudioCommand::Stop) | Err(_) => {
                        println!("Stopping playback");
                        stream.pause();
                        break;
                    }
                }
            }

            Ok(())
        });

        Ok(())
    }
}
