pub mod error;

pub use audio_command::AudioCommand;
pub use handle::AudioThreadHandle;

mod audio_command;
mod handle;

use crate::{
    audio_thread::error::AudioError,
    data_types::AudioContext,
    mixer::{Mixer, Project},
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{
    SharedRb,
    storage::Heap,
    traits::{Consumer, Producer, Split},
    wrap::caching::Caching,
};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};

pub struct AudioThread;

impl AudioThread {
    pub fn spawn(audio_ctx: AudioContext, initial_project: Project) -> AudioThreadHandle {
        let (command_tx, command_rx) = mpsc::channel();
        let (error_tx, error_rx) = mpsc::channel();
        let playhead = Arc::new(AtomicUsize::new(0));
        let playhead_clone = playhead.clone();

        thread::spawn(move || {
            AudioThread::audio_thread(
                command_rx,
                error_tx,
                playhead_clone,
                audio_ctx,
                initial_project,
            );
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
        audio_ctx: AudioContext,
        initial_project: Project,
    ) {
        let (mut producer, consumer) = ringbuf::HeapRb::<AudioCommand>::new(64).split();

        // Create a mixer with the given initial project
        let pending_project = Arc::new(Mutex::new(None));
        let pending_arc = Arc::clone(&pending_project);
        let mixer = Mixer::new(initial_project);

        // Get a cpal device
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Expect a default output device");

        // Manage is_playing using Arc
        let is_playing = Arc::new(AtomicBool::new(false));
        let is_playing_clone = is_playing.clone();

        // Create an output callback
        let config = cpal::StreamConfig {
            channels: audio_ctx.channels as u16,
            sample_rate: audio_ctx.sample_rate as u32,
            buffer_size: cpal::BufferSize::Fixed(audio_ctx.buffer_size as u32),
        };
        let stream = AudioThread::output_callback(
            mixer,
            consumer,
            pending_arc,
            device,
            config,
            playhead,
            is_playing_clone,
        );

        // Create a message loop
        for command in command_rx {
            match command {
                AudioCommand::Play => {
                    is_playing.store(true, Ordering::Release);
                }
                AudioCommand::Pause => {
                    is_playing.store(false, Ordering::Release);
                }
                AudioCommand::Seek(_) => {
                    if let Err(command) = producer.try_push(command) {
                        error_tx.send(AudioError::CommandFailed(command)).unwrap();
                    }
                }
                AudioCommand::UpdateProject(new_project) => {
                    println!("Received a new project");

                    let mut pending_project = pending_project.lock().unwrap();
                    *pending_project = Some(new_project);
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
        is_playing: Arc<AtomicBool>,
    ) -> cpal::Stream {
        device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    // Get the project without blocking
                    if let Ok(mut pending) = pending_project.try_lock()
                        && let Some(new_project) = pending.take()
                    {
                        mixer.apply_project(new_project);
                        println!("Updated to a new project");
                    }

                    // Process the seek
                    if let Some(command) = consumer.try_pop()
                        && let AudioCommand::Seek(target) = command
                    {
                        let target_sample = mixer.project.tempo_map.beats_to_samples(target);
                        playhead.store(target_sample, Ordering::Relaxed);
                        mixer.seek();
                    }

                    let is_playing = is_playing.load(Ordering::Relaxed);
                    if is_playing {
                        // Process the mixer
                        let current_playhead = playhead.load(Ordering::Relaxed);
                        mixer.process(current_playhead, data);

                        // Increment the playhead
                        playhead.fetch_add(mixer.project.audio_ctx.buffer_size, Ordering::Relaxed);
                    } else {
                        data.fill(0.0);
                    }
                },
                |err| {
                    eprintln!("An error occured on stream: {}", err);
                },
                None,
            )
            .expect("Failed to create a new stream")
    }
}
