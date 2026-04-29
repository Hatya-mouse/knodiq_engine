use crate::{
    audio_thread::AudioCommand,
    mixer::{Mixer, Project, TrackID},
};
use cpal::traits::DeviceTrait;
use ringbuf::{SharedRb, storage::Heap, traits::Consumer, wrap::caching::Caching};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

pub(super) fn output_callback(
    mut mixer: Mixer,
    mut consumer: Caching<Arc<SharedRb<Heap<AudioCommand>>>, false, true>,
    pending_project: Arc<Mutex<Option<Project>>>,
    device: cpal::Device,
    config: cpal::StreamConfig,
    playhead: Arc<AtomicUsize>,
    is_playing: Arc<AtomicBool>,
) -> cpal::Stream {
    let mut armed_track: Option<TrackID> = None;

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
                if let Some(command) = consumer.try_pop() {
                    match command {
                        AudioCommand::Seek(target) => {
                            let target_sample = mixer.project.tempo_map.beats_to_samples(target);
                            // Do not forget to update the current_playhead for processing later
                            current_playhead = target_sample;
                            playhead.store(target_sample, Ordering::Relaxed);
                            mixer.seek(target_sample);
                        }
                        AudioCommand::ArmTrack(track_id) => {
                            armed_track = Some(track_id);
                        }
                        AudioCommand::DisarmTrack => {
                            armed_track = None;
                        }
                        _ => (),
                    }
                }

                let is_playing = is_playing.load(Ordering::Relaxed);

                if is_playing {
                    // --- MIXER PROCESSING ---
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
