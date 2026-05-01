mod audio_command;
mod audio_thread;
mod export;
mod handle;
mod midi_thread;

pub use audio_command::{AudioCommand, AudioError, AudioResult, MidiCommand};
pub use handle::AudioThreadHandle;

use crate::{
    data_types::{AudioContext, MidiEvent},
    graph::error::GraphError,
    mixer::Project,
};
use ringbuf::{HeapRb, traits::Split};
use std::{
    sync::{Arc, atomic::AtomicUsize, mpsc},
    thread,
};

pub struct AudioThread;

impl AudioThread {
    pub fn spawn(
        audio_ctx: AudioContext,
        mut initial_project: Project,
    ) -> Result<AudioThreadHandle, GraphError> {
        // MPSC channels to send commands to the processing threads from the host.
        let (audio_command_tx, audio_command_rx) = mpsc::channel();
        let (midi_command_tx, midi_command_rx) = mpsc::channel();
        // MPSC channel to send the results back to the host.
        let (result_tx, result_rx) = mpsc::channel();
        // Shared playhead position using Arc and AtomicUsize for thread-safe access.
        let playhead = Arc::new(AtomicUsize::new(0));
        let playhead_clone = playhead.clone();
        // A ringbuf to send MIDI events to the audio thread from the midi thread.
        let (midi_producer, midi_consumer) = HeapRb::<MidiEvent>::new(64).split();
        // A ringbuf to send the calculated VU levels to the host.
        let (vu_producer, vu_consumer) = HeapRb::<f32>::new(audio_ctx.channels * 2).split();

        // Prepare the initial project
        initial_project.prepare()?;

        // --- MAIN AUDIO THREAD ---
        thread::spawn(move || {
            audio_thread::audio_thread(
                audio_command_rx,
                result_tx,
                midi_consumer,
                vu_producer,
                playhead_clone,
                audio_ctx,
                initial_project,
            );
        });

        // --- MIDI THREAD ---
        thread::spawn(move || midi_thread::midi_thread(midi_command_rx, midi_producer));

        Ok(AudioThreadHandle {
            audio_command_tx,
            midi_command_tx,
            result_rx,
            vu_consumer,
            playhead,
        })
    }
}
