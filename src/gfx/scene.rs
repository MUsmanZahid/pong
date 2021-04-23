#[derive(Debug)]
pub(crate) struct Glyph {
    pub(crate) num_vertices: usize,
    pub(crate) texture_index: usize,
    pub(crate) offset: usize,
    pub(crate) length: usize,
}

#[derive(Debug)]
pub(crate) struct SpriteData {
    pub(crate) index: usize,
    pub(crate) offset: usize,
    pub(crate) length: usize,
}

#[derive(Debug, Default)]
pub(crate) struct Scene {
    pub(crate) sprite: Vec<SpriteData>,
    pub(crate) glyphs: Vec<Glyph>,
    pub(crate) data: Vec<u8>,
}

impl Scene {
    pub(crate) fn insert_sprite(&mut self, sprite_data: &[f32], sprite_index: usize) {
        let offset = self.data.len();
        let len = sprite_data.len() * std::mem::size_of::<f32>();
        let byte_slice = {
            let slice =
                unsafe { std::slice::from_raw_parts(sprite_data.as_ptr() as *const _, len) };
            std::mem::ManuallyDrop::new(slice)
        };

        let sprite_data = SpriteData {
            index: sprite_index,
            offset,
            length: len,
        };
        self.sprite.push(sprite_data);
        self.data.extend_from_slice(&byte_slice);
    }
}
