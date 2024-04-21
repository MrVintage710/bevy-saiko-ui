

use bevy::{asset::{AssetLoader, AsyncReadExt}, math::U16Vec2, prelude::*, render::Extract, utils::{BoxedFuture, HashMap}};
use etagere::{euclid::{Box2D, UnknownUnit}, Allocation, AtlasAllocator, Size};
use thiserror::Error;
use ttf_parser::{Face, GlyphId};
use msdfgen::{Bitmap, FontExt, MsdfGeneratorConfig, Range, Rgb};

use super::SaikoCharacterSet;


pub struct SaikoFontSdfPlugin;

impl Plugin for SaikoFontSdfPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset_loader::<SaikoFontSdfLoader>()
            .init_asset::<SaikoFontSdf>()
        ;
    }
}

//==============================================================================
//             SaikoFont Systems
//==============================================================================

/// This system extracts fonts from the world and puts them into the correct font atlases
fn extract_sdf_fonts (
    fonts : Extract<Res<Assets<SaikoFontSdf>>>
    
) {
    for (id, font) in fonts.iter() {
        
    }
}

//==============================================================================
//             SaikoFontArray
//==============================================================================

const FONT_ATLAS_DIMS : u32 = 2048;

#[derive(Asset, TypePath, Clone)]
pub struct SaikoFontSdf {
    bitmap : Vec<Rgb<f32>>,
    allocator : AtlasAllocator,
    glyph_data : HashMap<char, SaikoGlyphData>,
    glyph_size : u32,
    is_dirty : bool,
}

impl Default for SaikoFontSdf {
    fn default() -> Self {
        let bitmap = vec![Rgb::new(0.0, 0.0, 0.0); FONT_ATLAS_DIMS as usize * FONT_ATLAS_DIMS as usize];
        let allocator = AtlasAllocator::new(Size::splat(FONT_ATLAS_DIMS as i32));
        let glyph_size = 32;
        
        Self { bitmap, allocator, glyph_size, glyph_data : HashMap::default(), is_dirty : true }
    }
}

impl SaikoFontSdf {
    pub fn add_glyphs(&mut self, font : &Face, characters : impl IntoIterator<Item = char>) {
        for character in characters.into_iter() {
            self.add_glyph(&font, character);
        }
    }
    
    pub fn add_glyph(&mut self, font : &Face, character : char) -> bool {
        let Some(glyph) = font.glyph_index(character) else { return false };
        let Some(mut glyph_shape) = font.glyph_shape(glyph) else { return false };
        let Some(allocation) = self.allocate_glyph(character, glyph, &font) else { return false };
        
        let bound = glyph_shape.get_bound();
        let Some(framing) = bound.autoframe(self.glyph_size, self.glyph_size, Range::Px(4.0), None)
            else { return false };
        
        let mut bitmap = Bitmap::new(self.glyph_size, self.glyph_size);
        
        glyph_shape.edge_coloring_simple(3.0, 0);
        glyph_shape.generate_msdf(&mut bitmap, framing, MsdfGeneratorConfig::default());
        bitmap.flip_y();
        
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
                let target_pixel = self.pixel_mut(x, y);
                *target_pixel = *source_pixel;
            }
        }
    }
    
    pub fn pixel(&self, x : u32, y : u32) -> &Rgb<f32> {
        &self.bitmap[((y*FONT_ATLAS_DIMS) + x) as usize]
    }
    
    fn pixel_mut(&mut self, x : u32, y : u32) -> &mut Rgb<f32> {
        self.is_dirty = true;
        &mut self.bitmap[((y*FONT_ATLAS_DIMS) + x) as usize]
    }
    
    fn allocate_glyph(&mut self, character : char, glyph_id : GlyphId, font : &Face) -> Option<Allocation> {
        let Some(allocation) = self.allocator.allocate(Size::splat(32)) else { return None };
        let rect = self.allocator.get(allocation.id);
        
        let data = SaikoGlyphData::from_font(glyph_id, font, allocation, rect);
        self.glyph_data.insert(character, data);
        Some(allocation)
    }
    
    pub fn glyph_data(&self, character : char) -> Option<&SaikoGlyphData> {
        self.glyph_data.get(&character)
    }
}

//==============================================================================
//             SaikoGlyphData
//==============================================================================

#[derive(Debug, Clone)]
pub struct SaikoGlyphData {
    allocation : Allocation,
    advance : U16Vec2,
    vertical_origin : i16,
    min : Vec2,
}

impl SaikoGlyphData {
    fn from_font(glyph_id : GlyphId, font : &Face, allocation : Allocation, rect : Box2D<i32, UnknownUnit>) -> Self {
        
        // Get Character Advance
        let (ha, va) = (font.glyph_hor_advance(glyph_id), font.glyph_ver_advance(glyph_id));
        let (ha, va) = (ha.unwrap_or(0), va.unwrap_or(0));
        let advance = U16Vec2::new(ha, va);
        
        let vertical_origin = font.glyph_y_origin(glyph_id).unwrap_or(0);
        
        let font_atlas_dims_float = FONT_ATLAS_DIMS as f32;
        let min = Vec2::new(rect.min.x as f32 / font_atlas_dims_float, rect.min.y as f32 / font_atlas_dims_float);
        // let max = Vec2::new(rect.max.x as f32 / font_atlas_dims_float, rect.max.y as f32 / font_atlas_dims_float);
        
        Self { advance, allocation, vertical_origin, min }
    }
    
    pub fn advance(&self) -> U16Vec2 {
        self.advance
    }

    pub fn allocation(&self) -> Allocation {
        self.allocation
    }

    pub fn vertical_origin(&self) -> i16 {
        self.vertical_origin
    }
    
    pub fn min(&self) -> Vec2 {
        self.min
    }
}

//==============================================================================
//             SaikoFontArray AssetLoader
//==============================================================================

#[derive(Default)]
struct SaikoFontSdfLoader;

impl AssetLoader for SaikoFontSdfLoader {
    type Asset = SaikoFontSdf;

    type Settings = ();

    type Error = SaikoSdfFontError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut data = Vec::new();
            reader.read_to_end(&mut data).await
                .expect(&format!("There was an error reading the font file {:?}", _load_context.path()));
            
            let Ok(font) = Face::parse(&data, 0) else { return Err(SaikoSdfFontError::Io) };
            let mut font_atlas = SaikoFontSdf::default();
            
            font_atlas.add_glyphs(&font, SaikoCharacterSet::ascii());
            
            Ok(font_atlas)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ttf"]
    }
}

//==============================================================================
//             SaikoFontError 
//==============================================================================

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SaikoSdfFontError {
    /// An [IO](std::io) Error
    #[error("Could not load asset")]
    Io,
}

//==============================================================================
//             SaikoFontE 
//==============================================================================

#[derive(Debug, Default)]
pub struct SaikoSdfFontSettings {
    pub character_set : SaikoCharacterSet,
}