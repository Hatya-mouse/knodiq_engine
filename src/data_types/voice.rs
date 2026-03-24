#[repr(C)]
#[derive(Clone)]
pub struct Voice {
    pub frequency: f32,
    pub velocity: f32,
    pub age: f32,
    pub is_active: bool,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            frequency: 0.0,
            velocity: 0.0,
            age: 0.0,
            is_active: false,
        }
    }
}

impl Voice {
    pub fn new(frequency: f32, velocity: f32, age: f32, is_active: bool) -> Self {
        Self {
            frequency,
            velocity,
            age,
            is_active,
        }
    }
}
