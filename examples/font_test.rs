//==============================================================================
//  This is just to test the process of generating the font MSDF atlas. I am 
//  leaving it as a simple example of how this is done, for maintainers. This 
//  may be inaccurate in the future.
//==============================================================================

use std::fs::File;
use bevy::utils::HashMap;
use etagere::{euclid::Box2D, Allocation, AtlasAllocator, Size};
use notosans::REGULAR_TTF as FONT;
use ttf_parser::{Face, GlyphId};
use msdfgen::{Bitmap, FillRule, FontExt, Gray, MsdfGeneratorConfig, Range, Rgb, MID_VALUE};

struct FontAtlas {
    bitmap : Bitmap<Rgb<f32>>,
    allocator : AtlasAllocator,
    allocated_glyphs : HashMap<char, Allocation>,
    glyph_size : u32,
}

impl Default for FontAtlas {
    fn default() -> Self {
        let bitmap = Bitmap::new(2048, 2048);
        let allocator = AtlasAllocator::new(Size::splat(2048));
        let glyph_size = 32;
        
        Self { bitmap, allocator, glyph_size, allocated_glyphs : HashMap::default() }
    }
}

impl FontAtlas {
    pub fn add_new_glyph(&mut self, character : char, font : &Face) -> bool {
        let Some(glyph) = font.glyph_index(character) else { return false };
        let Some(mut glyph_shape) = font.glyph_shape(glyph) else { return false };
        
        let bound = glyph_shape.get_bound();
        let Some(framing) = bound.autoframe(self.glyph_size, self.glyph_size, Range::Px(4.0), None)
            else { return false };
        
        let mut bitmap = Bitmap::new(self.glyph_size, self.glyph_size);
        
        glyph_shape.edge_coloring_simple(3.0, 0);
        
        glyph_shape.generate_msdf(&mut bitmap, framing, MsdfGeneratorConfig::default());
        
        bitmap.flip_y();
        
        let Some(allocation) = self.allocator.allocate(Size::splat(32)) else { return false };

        self.insert_bitmap(bitmap, allocation);
        
        true
    }
    
    fn insert_bitmap(&mut self, bitmap : Bitmap<Rgb<f32>>, allocation : Allocation) {
        let rect = self.allocator.get(allocation.id);
        
        for y in 0..bitmap.height() {
            for x in 0..bitmap.width() {
                let source_pixel = bitmap.pixel(x, y);
                let x = x + rect.min.x as u32;
                let y = y + rect.min.y as u32;
                let target_pixel = self.bitmap.pixel_mut(x, y);
                *target_pixel = *source_pixel;
            }
        }
    }
    
    pub fn output_to_file(&self, filename : &str) {
        let mut output = File::create(filename).unwrap();
        self.bitmap.write_png(&mut output).unwrap();
    }
}

pub fn main() {
    let font = Face::from_slice(&FONT, 0).unwrap();
    let mut font_atlas = FontAtlas::default();
    
    for character in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-~!.,'/\\\"".chars() {
        font_atlas.add_new_glyph(character, &font);
    }
    
    font_atlas.output_to_file("font_atlas.png");
}