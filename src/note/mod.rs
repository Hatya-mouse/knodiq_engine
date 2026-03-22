#[repr(C)]
#[derive(Clone, Debug)]
pub struct KaslNote {
    pub frequency: f32,
    pub velocity: f32,
    pub is_active: bool,
}
