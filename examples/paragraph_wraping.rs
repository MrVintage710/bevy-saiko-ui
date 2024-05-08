//This example shows how SaikoUi Wraps its paragraphs.

use bevy::utils::HashMap;
use fit_text::{BasicGlyphs, CharacterWidthCache, TextFormat};
use paragraph_breaker::{Item, INFINITE_PENALTY};
use ttf_parser::{Face, GlyphId};
use notosans::REGULAR_TTF as FONT;

pub fn main() {
    // let Some(mut glyphs) = BasicGlyphs::from_bytes(FONT) else { return; };
    let font = Face::parse(FONT, 0).expect("Problem parsing font.");
    let mut font_metrics = FontMetrics::from_font(&font, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-~!.,'/\\\"".chars());
    
    let result = font_metrics.justify_text("This is a test to see how a line is split!", ((0.0, 0.0), (200.0, 200.0)), TextFormat::new(32));
    
    println!("{:?}", result);
}

struct FontMetrics {
    space_width : f64,
    character_metrics : HashMap<char, GlyphMetrics>
}

impl CharacterWidthCache for FontMetrics {
    fn char_width(&mut self, character: char, font_size: u32) -> f64 {
        match character {
            ' ' => self.space_width * font_size as f64,
            _ => {
                let Some(glyph_metrics) = self.character_metrics.get(&character) else { return 0.0 };
                glyph_metrics.h_advance as f64 * font_size as f64
            }
        }
    }
}

impl FontMetrics {
    pub fn from_font(font : &Face, characters : impl IntoIterator<Item = char>) -> Self {
        let mut character_metrics = HashMap::new();
        
        for c in characters.into_iter() {
            let Some(glyph_id) = font.glyph_index(c) else { continue };
            
            let font_height = font.height() as f64;
            let h_advance = (font.glyph_hor_advance(glyph_id).unwrap_or(0) as f64) / font_height;
            let v_advance = (font.glyph_ver_advance(glyph_id).unwrap_or(0) as f64) / font_height;
            let rect = font.glyph_bounding_box(glyph_id).unwrap();
            let width = rect.width() as f64 / font_height;
            let height = rect.height() as f64 / font_height;
            let x = rect.x_min as f64 / font_height;
            let y = rect.y_min as f64 / font_height;
    
            character_metrics.insert(c, GlyphMetrics {
                h_advance,
                v_advance,
                x,
                y,
                width,
                height,
            });
        }
        
        let space_width = character_metrics.get(&' ').map(|m| m.h_advance).unwrap_or(0.0);
        
        FontMetrics {
            space_width,
            character_metrics
        }
    }
    
    pub fn glyph_metrics(&self, c : char) -> Option<&GlyphMetrics> {
        self.character_metrics.get(&c)
    }
}

struct GlyphMetrics {
    h_advance : f64,
    v_advance : f64,
    x : f64,
    y : f64,
    width : f64,
    height : f64,
}

