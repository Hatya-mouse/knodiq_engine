#[derive(Default)]
pub struct TypeInfo {
    pub size: usize,
    pub align: usize,
}

impl TypeInfo {
    pub fn new(size: usize, align: usize) -> Self {
        Self { size, align }
    }
}
