#[repr(C)]
#[derive(Clone, Debug)]
pub struct Voice {
    pub frequency: f32,
    pub velocity: f32,
    pub is_active: bool,
}
