// output_device_manager.rs
// Manages output devices and process the audio to meet the device configuration
// © 2025 Shuntaro Kasatani

use crate::audio_engine::resample::AudioResampler;
use crate::audio_engine::source::AudioSource;
use cpal::traits::DeviceTrait;

pub struct OutputDeviceManager {}

impl OutputDeviceManager {
    pub fn new() -> Self {
        Self {}
    }

    /// Get the available output devices
    // pub fn get_devices(&self) -> Result<cpal::Devices, Box<dyn std::error::Error>> {
    //     // If you're filtering devices somewhere, collect them first
    //     let devices = cpal::default_host().output_devices()?;
    //     Ok(devices)
    // }

    /// Process the audio source that meets the device's configuration
    pub fn process_audio(
        &self,
        device: &cpal::Device,
        source: &AudioSource,
    ) -> Result<AudioSource, Box<dyn std::error::Error>> {
        let device_config = device.default_output_config()?;
        let resampler = AudioResampler::new();
        let resampled_source =
            resampler.resample(source, device_config.sample_rate().0 as usize)?;
        Ok(resampled_source)
    }
}
