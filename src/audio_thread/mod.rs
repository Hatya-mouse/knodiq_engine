mod audio_command;
pub mod error;
mod handle;

pub use audio_command::AudioCommand;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
pub use handle::AudioThreadHandle;
use ringbuf::{
    SharedRb,
    storage::Heap,
    traits::{Consumer, Producer, Split},
    wrap::caching::Caching,
};

use crate::{
    audio_thread::error::AudioError,
    data_types::AudioContext,
    mixer::{Mixer, Project},
};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};

pub struct AudioThread;

impl AudioThread {
    pub fn spawn() -> AudioThreadHandle {
        let (command_tx, command_rx) = mpsc::channel();
        let (error_tx, error_rx) = mpsc::channel();
        let playhead = Arc::new(AtomicUsize::new(0));
        let playhead_clone = playhead.clone();

        thread::spawn(move || {
            AudioThread::audio_thread(command_rx, error_tx, playhead_clone);
        });

        AudioThreadHandle {
            command_tx,
            error_rx,
            playhead,
        }
    }

    fn audio_thread(
        command_rx: mpsc::Receiver<AudioCommand>,
        error_tx: mpsc::Sender<AudioError>,
        playhead: Arc<AtomicUsize>,
    ) {
        let (mut producer, consumer) = ringbuf::HeapRb::<AudioCommand>::new(64).split();

        // Create a mixer and a project
        let audio_ctx = AudioContext {
            channels: 2,
            sample_rate: 48000,
            buffer_size: 512,
            max_voices: 32,
        };
        let pending_project = Arc::new(Mutex::new(None));
        let mixer = Mixer::new(Project::new(audio_ctx.clone(), 120.0));

        // Get a cpal device
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Expect a default output device");

        // Create an output callback
        let config = cpal::StreamConfig {
            channels: audio_ctx.channels as u16,
            sample_rate: audio_ctx.sample_rate as u32,
            buffer_size: cpal::BufferSize::Fixed(audio_ctx.buffer_size as u32),
        };
        let stream = AudioThread::output_callback(
            mixer,
            consumer,
            pending_project,
            device,
            config,
            playhead,
        );

        // Create a message loop
        for command in command_rx {
            match command {
                AudioCommand::Play => {
                    stream.play();
                }
                AudioCommand::Pause => {
                    stream.pause();
                }
                _ => (),
            }

            match producer.try_push(command) {
                Ok(_) => (),
                Err(command) => {
                    error_tx.send(AudioError::CommandFailed(command));
                }
            }
        }
    }

    fn output_callback(
        mut mixer: Mixer,
        mut consumer: Caching<Arc<SharedRb<Heap<AudioCommand>>>, false, true>,
        pending_project: Arc<Mutex<Option<Project>>>,
        device: cpal::Device,
        config: cpal::StreamConfig,
        playhead: Arc<AtomicUsize>,
    ) -> cpal::Stream {
        device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    if let Ok(mut pending) = pending_project.try_lock() {
                        if let Some(new_project) = pending.take() {
                            mixer.apply_project(new_project);
                        }
                    }

                    // Process the seek
                    if let Some(command) = consumer.try_pop()
                        && let AudioCommand::Seek(target) = command
                    {
                        let target_sample = mixer.project.tempo_map.beats_to_samples(target);
                        playhead.store(target_sample, Ordering::Relaxed);
                        mixer.seek();
                    }

                    // Process the mixer
                    let current_playhead = playhead.load(Ordering::Relaxed);
                    mixer.process(current_playhead, data.as_mut_ptr() as *mut u8);

                    // Increment the playhead
                    playhead.fetch_add(mixer.project.audio_ctx.buffer_size, Ordering::Relaxed);
                },
                |err| {
                    eprintln!("An error occured on stream: {}", err);
                },
                None,
            )
            .expect("Failed to create a new stream")
    }
}
