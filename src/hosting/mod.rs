// hosting.rs
// Provides functionality for loading and interacting with LV2 plugins.
// © 2025 Shuntaro Kasatani

use libloading::{Library, Symbol};
use std::ffi::c_void;

type Lv2DescriptorFn = unsafe extern "C" fn(u32) -> *const c_void;

pub fn load() {
    let plugin_path = "/Library/Audio/Plug-Ins/LV2/Vital.lv2/Vital.so";
    unsafe {
        match load_lv2_descriptor(plugin_path, 0) {
            Ok(descriptor) => {
                println!("Loaded plugin descriptor ⭐️: {:?}", descriptor);
            }
            Err(err) => {
                eprintln!("Failed to load plugin descriptor 🤨: {}", err);
            }
        }
    }
}

unsafe fn load_lv2_descriptor(
    library_path: &str,
    index: u32,
) -> Result<*const c_void, Box<dyn std::error::Error>> {
    // Load the library using libloading
    let lib = Library::new(library_path)?;
    // Get the lv2_descriptor function
    let descriptor_fn: Symbol<Lv2DescriptorFn> = lib.get(b"lv2_descriptor")?;
    // Run the function and get the descriptor
    let descriptor = descriptor_fn(index);
    if descriptor.is_null() {
        return Err("Failed to load plugin descriptor. It may not exist.".into());
    }
    // Return the descriptor
    Ok(descriptor)
}
