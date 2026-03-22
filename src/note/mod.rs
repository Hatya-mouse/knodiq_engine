#[repr(C)]
#[derive(Clone)]
pub struct KaslNote {
    pub frequency: f32,
    pub velocity: f32,
    pub is_active: bool,
}
