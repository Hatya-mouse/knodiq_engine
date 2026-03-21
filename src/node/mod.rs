use crate::audio_context::AudioContext;

pub trait Node: Send {
    fn prepare(&mut self, audio_ctx: &AudioContext);

    fn process(&mut self, inputs: &[*const u8], outputs: &[*mut u8], audio_ctx: &AudioContext);
}
