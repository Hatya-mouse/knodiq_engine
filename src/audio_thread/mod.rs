mod audio_command;
mod callback;
mod export;
mod handle;

pub use audio_command::{AudioCommand, AudioError, AudioResult};
pub use handle::AudioThreadHandle;

use crate::{
    data_types::AudioContext,
    graph::error::GraphError,
    mixer::{Mixer, Project},
};
use cpal::traits::{HostTrait, StreamTrait};
use ringbuf::traits::{Producer, Split};
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
        let stream = callback::output_callback(
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
                            *pending_arc.lock().unwrap() = Some(*new_project);
                        }
                    });
                }
                AudioCommand::ExportAudio(project) => {
                    let result_tx = result_tx.clone();
                    export::spawn_export_thread(result_tx, *project);
                }
                AudioCommand::ArmTrack(_) => {
                    if let Err(command) = producer.try_push(command) {
                        result_tx
                            .send(Err(AudioError::CommandFailed(command)))
                            .unwrap();
                    }
                }
                AudioCommand::DisarmTrack => {
                    if let Err(command) = producer.try_push(command) {
                        result_tx
                            .send(Err(AudioError::CommandFailed(command)))
                            .unwrap();
                    }
                }
            }
        }

        drop(stream);
    }
}
