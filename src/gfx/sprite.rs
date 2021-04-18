use crate::{ffi::vk, math::Vector2};

pub(crate) struct Sprite {
    pub(crate) texture_index: usize,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Sprite {
    pub(crate) fn generate_vertex_data(
        &self,
        position: Vector2,
        extent: vk::Extent2D,
    ) -> [f32; 24] {
        let (width, height) = self.pixels_to_ndc(extent);

        let mut data = [0.0; 24];
        // Bottom-left vertex
        data[0] = position.x - width;
        data[1] = -position.y + height;
        data[2] = 0.0;
        data[3] = 1.0;

        // Top-left vertex
        data[4] = position.x - width;
        data[5] = -position.y - height;
        data[6] = 0.0;
        data[7] = 0.0;

        // Top-right vertex
        data[8] = position.x + width;
        data[9] = -position.y - height;
        data[10] = 1.0;
        data[11] = 0.0;

        // Bottom-left vertex
        data[12] = position.x - width;
        data[13] = -position.y + height;
        data[14] = 0.0;
        data[15] = 1.0;

        // Top-right vertex
        data[16] = position.x + width;
        data[17] = -position.y - height;
        data[18] = 1.0;
        data[19] = 0.0;

        // Bottom-right vertex
        data[20] = position.x + width;
        data[21] = -position.y + height;
        data[22] = 1.0;
        data[23] = 1.0;
        return data;
    }

    pub(crate) fn pixels_to_ndc(&self, extent: vk::Extent2D) -> (f32, f32) {
        return (
            self.width as f32 / extent.width as f32,
            self.height as f32 / extent.height as f32,
        );
    }
}
