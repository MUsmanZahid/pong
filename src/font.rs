use std::ffi::CStr;

use freetype::freetype::{
    FT_Done_Face, FT_Done_FreeType, FT_Get_Char_Index, FT_Init_FreeType, FT_Load_Glyph,
    FT_New_Face, FT_Render_Glyph, FT_Set_Char_Size,
};

pub(crate) const NUM_ASCII_GLYPHS: usize = 95;

#[derive(Clone, Debug)]
pub(crate) struct CoverageMap {
    pub(crate) data: Box<[u8]>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Glyph {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) offset_x: i32,
    pub(crate) offset_y:i32,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct GlyphUV {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) offset_x: f32,
    pub(crate) offset_y: f32,
    pub(crate) width_uv: f32,
    pub(crate) height_uv: f32
}

pub(crate) fn generate_bitmap(
    font: &CStr,
    height: u32,
) -> ([GlyphUV; NUM_ASCII_GLYPHS], CoverageMap) {
    let (library, face) = unsafe {
        let mut library = std::mem::zeroed();
        FT_Init_FreeType(&mut library);

        let mut face = std::ptr::null_mut();
        FT_New_Face(library, font.as_ptr(), 0, &mut face);
        FT_Set_Char_Size(face, 0, height as i64 * 64, 192, 192);

        (library, face)
    };

    let face_metrics = unsafe { (*(*face).size).metrics };
    let line_height = (face_metrics.height / 64) as u32;
    let mut bitmap = CoverageMap::new(
        CoverageMap::get_maximum_row_width(face),
        5 * line_height + (-face_metrics.descender / 64) as u32,
    );
    
    let glyphs = bitmap.render_ascii_glyphs(face);
    let mut glyphs_uv = [unsafe { std::mem::zeroed() }; NUM_ASCII_GLYPHS];
    for (glyph_uv, glyph) in glyphs_uv.iter_mut().zip(glyphs.iter()) {
        *glyph_uv = GlyphUV {
            width: glyph.width,
            height: glyph.height,
            offset_x: glyph.offset_x as f32 / bitmap.width as f32,
            offset_y: glyph.offset_y as f32 / bitmap.height as f32,
            width_uv: glyph.width as f32 / bitmap.width as f32,
            height_uv: glyph.height as f32 / bitmap.height as f32,
        };
    }

    unsafe {
        FT_Done_Face(face);
        FT_Done_FreeType(library);
    }

    return (glyphs_uv, bitmap);
}

impl CoverageMap {
    fn draw_glyph(&mut self, glyph: &Glyph, data: *mut u8) {
        let mut offset;
        for i in 0..glyph.height {
            for j in 0..glyph.width {
                let hoff = j as i32 + glyph.offset_x;
                let voff = i as i32 + glyph.offset_y;
                offset = (voff * self.width as i32 + hoff) as usize;

                self.data[offset] = unsafe { *data.add((i * glyph.width + j) as usize) };
            }
        }
    }

    fn get_maximum_row_width(face: freetype::freetype::FT_Face) -> u32 {
        let mut max_row_width = 0;
        let mut glyph_index;

        for i in 0..5 {
            let mut row_extent = 0;
            for j in 0..19 {
                glyph_index = unsafe { FT_Get_Char_Index(face, (i * 19 + j) + 0x20) };
                unsafe {
                    FT_Load_Glyph(
                        face,
                        glyph_index,
                        freetype::freetype::FT_LOAD_NO_BITMAP as i32,
                    )
                };
                row_extent += unsafe { (*(*face).glyph).metrics.horiAdvance / 64 } as u32;
            }

            if max_row_width < row_extent {
                max_row_width = row_extent;
            }
        }

        return max_row_width;
    }

    fn new(width: u32, height: u32) -> Self {
        let data = vec![0; (width * height) as usize].into_boxed_slice();

        let map = Self {
            data,
            width,
            height,
        };
        return map;
    }

    #[cfg(debug_assertions)]
    pub fn print_pgm(&self) {
        println!("P2\n{} {}\n255", self.width, self.height);
        (0..self.height).for_each(|i| {
            (0..self.width).for_each(|j| {
                let offset = i * self.width + j;
                print!("{} ", self.data[offset as usize]);
            });
            println!();
        });
    }

    fn render_ascii_glyphs(
        &mut self,
        face: freetype::freetype::FT_Face,
    ) -> [Glyph; NUM_ASCII_GLYPHS] {
        // dbg!(&self);
        let mut glyphs: [Glyph; NUM_ASCII_GLYPHS] =
            [unsafe { std::mem::zeroed() }; NUM_ASCII_GLYPHS];
        let face_metrics = unsafe { (*(*face).size).metrics };

        let mut bitmap;
        let mut glyph_index;
        let mut glyph_metrics;
        let mut offset;
        for i in 0..5 {
            let mut current_advance: u32 = 0;

            for j in 0..19 {
                offset = i * 19 + j;
                glyph_index = unsafe { FT_Get_Char_Index(face, offset + 0x20) };
                unsafe {
                    FT_Load_Glyph(
                        face,
                        glyph_index,
                        freetype::freetype::FT_LOAD_DEFAULT as i32,
                    );
                    FT_Render_Glyph(
                        (*face).glyph,
                        freetype::freetype::FT_Render_Mode::FT_RENDER_MODE_NORMAL,
                    );
                }

                glyph_metrics = unsafe { (*(*face).glyph).metrics };
                bitmap = unsafe { (*(*face).glyph).bitmap };

                if offset == 0 {
                    glyphs[0].width = (glyph_metrics.horiAdvance / 64) as u32;
                    glyphs[0].height =
                        ((face_metrics.ascender - face_metrics.descender) / 64) as u32;
                    glyphs[0].offset_x = (glyph_metrics.horiAdvance / 64) as i32;
                    glyphs[0].offset_y = 0;
                } else {
                    let h_gap = (glyph_metrics.horiBearingX / 64) as i32;
                    let v_gap = ((face_metrics.ascender - glyph_metrics.horiBearingY) / 64) as i32;

                    glyphs[offset as usize] = Glyph {
                        width: bitmap.width,
                        height: bitmap.rows,
                        offset_x: current_advance as i32 + h_gap,
                        offset_y: i as i32 * (face_metrics.height / 64) as i32 + v_gap,
                    };

                    self.draw_glyph(&glyphs[offset as usize], bitmap.buffer);

                    glyphs[offset as usize].width = (glyph_metrics.horiAdvance / 64) as u32;
                    glyphs[offset as usize].height = (face_metrics.height / 64) as u32;
                    glyphs[offset as usize].offset_x = current_advance as i32;
                    glyphs[offset as usize].offset_y = (i as i64 * face_metrics.height / 64) as i32;
                }

                current_advance += (glyph_metrics.horiAdvance / 64) as u32;
            }
        }

        return glyphs;
    }
}
