pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn new(texture: u32) -> Self {
        Self { id: texture }
    }
}
