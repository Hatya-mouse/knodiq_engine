use crate::audio_context::AudioContext;

pub trait Node: Send {
    fn get_input_names(&self) -> Vec<String>;

    fn get_output_names(&self) -> Vec<String>;

    fn prepare(&mut self, audio_ctx: &AudioContext);

    fn process(&mut self, inputs: &[*const u8], outputs: &[*mut u8], audio_ctx: &AudioContext);
}
