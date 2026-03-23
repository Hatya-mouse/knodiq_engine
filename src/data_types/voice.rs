#[repr(C)]
#[derive(Clone)]
pub struct Voice {
    pub frequency: f32,
    pub velocity: f32,
    pub is_active: bool,
    pub elapsed_samples: i32,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            frequency: 0.0,
            velocity: 0.0,
            is_active: false,
            elapsed_samples: 0,
        }
    }
}

impl Voice {
    pub fn new(frequency: f32, velocity: f32, is_active: bool, elapsed_samples: i32) -> Self {
        Self {
            frequency,
            velocity,
            is_active,
            elapsed_samples,
        }
    }
}
