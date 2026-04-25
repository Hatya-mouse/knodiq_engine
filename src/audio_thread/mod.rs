pub use audio_command::AudioCommand;
pub use handle::AudioThreadHandle;

mod audio_command;
mod export;
mod handle;

use crate::{
    audio_thread::audio_command::{AudioError, AudioResult},
    data_types::AudioContext,
    graph::error::GraphError,
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
    pub fn spawn(
        audio_ctx: AudioContext,
        mut initial_project: Project,
    ) -> Result<AudioThreadHandle, GraphError> {
        let (command_tx, command_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let playhead = Arc::new(AtomicUsize::new(0));
        let playhead_clone = playhead.clone();

        // Prepare the initial project
        initial_project.prepare()?;

        thread::spawn(move || {
            AudioThread::audio_thread(
                command_rx,
                result_tx,
                playhead_clone,
                audio_ctx,
                initial_project,
            );
        });

        Ok(AudioThreadHandle {
            command_tx,
            result_rx,
            playhead,
        })
    }

    fn audio_thread(
        command_rx: mpsc::Receiver<AudioCommand>,
        result_tx: mpsc::Sender<Result<AudioResult, AudioError>>,
        playhead: Arc<AtomicUsize>,
        audio_ctx: AudioContext,
        initial_project: Project,
    ) {
        let (mut producer, consumer) = ringbuf::HeapRb::<AudioCommand>::new(64).split();

        // Create a mixer with the given initial project
        let pending_project = Arc::new(Mutex::new(None));
        let pending_arc = Arc::clone(&pending_project);
        let mixer = Mixer::new(initial_project);

        // Create a generation variable to track the latest prepared project
        let generation = Arc::new(AtomicUsize::new(0));

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

        if let Err(err) = stream.play() {
            result_tx
                .send(Err(AudioError::PlayStreamError(err)))
                .unwrap();
        }

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
                        result_tx
                            .send(Err(AudioError::CommandFailed(command)))
                            .unwrap();
                    }
                }
                AudioCommand::UpdateProject(mut new_project) => {
                    // Increment the current generation by one to mark it as the latest
                    let current_gen = generation.fetch_add(1, Ordering::SeqCst) + 1;
                    let gen_arc = Arc::clone(&generation);
                    let pending_arc = Arc::clone(&pending_project);
                    let result_tx = result_tx.clone();
                    std::thread::spawn(move || {
                        // Prepare the project before applying the project
                        if let Err(err) = new_project.prepare() {
                            result_tx.send(Err(AudioError::GraphError(err))).unwrap();
                            return;
                        }

                        // Check if the project is the latest one
                        if gen_arc.load(Ordering::SeqCst) == current_gen {
                            // Send the new project to the audio playback thread
                            *pending_arc.lock().unwrap() = Some(new_project);
                        }
                    });
                }
                AudioCommand::ExportAudio(project) => {
                    let result_tx = result_tx.clone();
                    export::spawn_export_thread(result_tx, project);
                }
            }
        }

        drop(stream);
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
                    let mut current_playhead = playhead.load(Ordering::Relaxed);

                    // Get the project without blocking
                    if let Ok(mut pending) = pending_project.try_lock()
                        && let Some(new_project) = pending.take()
                    {
                        mixer.apply_project(new_project, current_playhead);
                    }

                    // Process the seek
                    if let Some(command) = consumer.try_pop()
                        && let AudioCommand::Seek(target) = command
                    {
                        let target_sample = mixer.project.tempo_map.beats_to_samples(target);
                        // Do not forget to update the current_playhead for processing later
                        current_playhead = target_sample;
                        playhead.store(target_sample, Ordering::Relaxed);
                        mixer.seek(target_sample);
                    }

                    let is_playing = is_playing.load(Ordering::Relaxed);
                    if is_playing {
                        // Process the mixer
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
