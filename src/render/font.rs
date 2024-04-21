//==============================================================================
//  this file contains the logic for loading a font in and creating SDF data
//  for the font. It uses a library for doing this called msdfgen. In the future,
//  there maybe a custom implementation of this, but for now, this is the best.
//==============================================================================

use bevy::{asset::{AssetLoader, AsyncReadExt}, math::U16Vec2, prelude::*, render::{render_asset::{RenderAsset, RenderAssetUsages}, Extract, RenderApp}, utils::{BoxedFuture, HashMap}};
use etagere::{euclid::{Box2D, UnknownUnit}, Allocation, AtlasAllocator, Size};
use thiserror::Error;
use ttf_parser::{Face, GlyphId, OutlineBuilder};
use msdfgen::{Bitmap, FontExt, MsdfGeneratorConfig, Range, Rgb};

//==============================================================================
//             SaikoFontPlugin
//==============================================================================

pub struct SaikoFontPlugin;

impl Plugin for SaikoFontPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset_loader::<SaikoFontSdfLoader>()
            .init_asset::<SaikoFontSdf>()
        ;
        
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        
        
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

pub const FONT_ATLAS_DIMS : u32 = 2048;

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

//==============================================================================
//             Processing Vecotor Maps
//==============================================================================

// fn add_font_to_sdf_texture(font : Face, character_set : SaikoCharacterSet, character_scale : f32, coverage_threshold : f32) {

//     let mut image = Image::new(
//         Extent3d {
//             width: 2048,
//             height: 2048,
//             depth_or_array_layers: 256,
//         },
//         TextureDimension::D3,
//         vec![],
//         TextureFormat::Rgba32Float,
//         RenderAssetUsages::all(),
//     );
    
    
//     for character in character_set.characters {
//         println!("Character: {character}");
        
//         let Some(glyph_id) = font.glyph_index(character) else { println!("Font doesn't have this character."); continue };
//         let Some(mut shape) = font.glyph_shape(glyph_id) else { continue };
        
//         let bound = shape.get_bound();
//         let Some(framing) = bound.autoframe(32, 32, Range::Px(4.0), None) else { continue };
        
//         // let mut builder = GlyphBuilder::default();
//         // let Some(rect) = font.outline_glyph(glyph_id, &mut builder) else { continue };
//         // let glyph_outlines = builder.finalize();
        
//         let mut bitmap = Bitmap::new(32, 32);
        
//         shape.edge_coloring_simple(3.0, 0);
//         shape.generate_msdf(&mut bitmap, &framing, &MsdfGeneratorConfig::default());
        
//         // 16 / y1 = 100 / 200 => y1 / 16 = 200 / 100 => y1 = 16 * 200 / 100
//         // x1 / 16 = 100 / 200 => x1 = 16 * 100 / 200
        
//         println!("{:?}", bitmap.pixels());
//     }
// }

//==============================================================================
//             GlyphBuilder
//==============================================================================

#[derive(Default)]
struct GlyphBuilder {
    lines : Vec<GlyphCurve>,
    current_pos : Vec2
}

impl GlyphBuilder {
    fn finalize(mut self) -> Vec<GlyphCurve> {
        
        
        // self
        //     .lines.iter_mut()
        //     .for_each(|outline| outline.normalize(&self.dims))
        // ;
        
        self.lines
    }
}

impl OutlineBuilder for GlyphBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        
        
        self.current_pos.x = x;
        self.current_pos.y = y;
        
        // println!("M {:?} {:?}", self.current_pos.x, self.current_pos.y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let p0 = self.current_pos;
        let p2 = Vec2::new(x, y);
        let p1 = p2 * 0.5 - p0 * 0.5 + p0;
        self.lines.push(GlyphCurve{
            start : p0, 
            control : p1, 
            end : p2
        });
        self.move_to(x, y);
        
        // println!("M {} {} Q {} {} {x} {y} Z", p0.x, p0.y, p1.x, p1.y)
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.lines.push(GlyphCurve {
            start : self.current_pos.clone(), 
            control : Vec2::new(x1, y1), 
            end : Vec2::new(x, y)
        });
        // println!("M {} {} Q {} {} {x} {y} Z", self.current_pos.x, self.current_pos.y, x1, y1);
        self.move_to(x, y);
        
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let q0 = self.current_pos;
        let q1 = Vec2::new(x1, y1);
        let q2 = Vec2::new(x2, y2);
        let q3 = Vec2::new(x, y);
        let quarter = 1.0 / 4.0;
        let three_quarters = 3.0 / 4.0;
        let p1 = -quarter * q0 + three_quarters * q1 + three_quarters * q2 - quarter * q3;
        
        self.lines.push(GlyphCurve {
            start : q0, 
            control : p1, 
            end : q3
        });
        self.move_to(x, y)
    }

    fn close(&mut self) {
        self.current_pos.x = 0.0;
        self.current_pos.y = 0.0;
    }
}

// #[derive(PartialEq, Debug, Clone)]
// enum GlyphOutline {
//     Line(Vec2, Vec2),
//     Curve(Vec2, Vec2, Vec2),
// }

#[derive(Debug, Clone, Copy)]
struct GlyphCurve {
    start : Vec2,
    control : Vec2,
    end : Vec2
}

// impl GlyphOutline {
//     fn normalize (&mut self, scale : &Vec2) {
//         match self {
//             GlyphOutline::Line(start, end) => {
//                 *start = *start / *scale;
//                 *end = *end / *scale;
//             },
//             GlyphOutline::Curve(start, control, end) => {
//                 *start = *start / *scale;
//                 *control = *control / *scale;
//                 *end = *end / *scale;
//             }
//         }
//     }
// }

//==============================================================================
//             SaikoCharacterSet
//==============================================================================

#[derive(Debug, Clone)]
pub struct SaikoCharacterSet {
    characters : Vec<char>
}

impl Default for SaikoCharacterSet {
    fn default() -> Self {
        SaikoCharacterSet::ascii()
    }
}

impl From<Vec<char>> for SaikoCharacterSet {
    fn from(characters: Vec<char>) -> Self {
        SaikoCharacterSet {
            characters
        }
    }
}

impl From<&str> for SaikoCharacterSet {
    fn from(characters: &str) -> Self {
        SaikoCharacterSet {
            characters : characters.chars().collect()
        }
    }
}

impl From<&[char]> for SaikoCharacterSet {
    fn from(characters: &[char]) -> Self {
        SaikoCharacterSet {
            characters : characters.to_vec()
        }
    }
}

impl SaikoCharacterSet {
    pub fn ascii() -> SaikoCharacterSet {
        SaikoCharacterSet {
            characters : vec![
                '\u{00}', '\u{01}', '\u{02}', '\u{03}', '\u{04}', '\u{05}', '\u{06}', '\u{07}', '\u{08}', '\u{09}', '\u{0A}', '\u{0B}', '\u{0C}', '\u{0D}', '\u{0E}', '\u{0F}',
                '\u{10}', '\u{11}', '\u{12}', '\u{13}', '\u{14}', '\u{15}', '\u{16}', '\u{17}', '\u{18}', '\u{19}', '\u{1A}', '\u{1B}', '\u{1C}', '\u{1D}', '\u{1E}', '\u{1F}',
                '\u{20}', '\u{21}', '\u{22}', '\u{23}', '\u{24}', '\u{25}', '\u{26}', '\u{27}', '\u{28}', '\u{29}', '\u{2A}', '\u{2B}', '\u{2C}', '\u{2D}', '\u{2E}', '\u{2F}',
                '\u{30}', '\u{31}', '\u{32}', '\u{33}', '\u{34}', '\u{35}', '\u{36}', '\u{37}', '\u{38}', '\u{39}', '\u{3A}', '\u{3B}', '\u{3C}', '\u{3D}', '\u{3E}', '\u{3F}',
                '\u{40}', '\u{41}', '\u{42}', '\u{43}', '\u{44}', '\u{45}', '\u{46}', '\u{47}', '\u{48}', '\u{49}', '\u{4A}', '\u{4B}', '\u{4C}', '\u{4D}', '\u{4E}', '\u{4F}',
                '\u{50}', '\u{51}', '\u{52}', '\u{53}', '\u{54}', '\u{55}', '\u{56}', '\u{57}', '\u{58}', '\u{59}', '\u{5A}', '\u{5B}', '\u{5C}', '\u{5D}', '\u{5E}', '\u{5F}',
                '\u{60}', '\u{61}', '\u{62}', '\u{63}', '\u{64}', '\u{65}', '\u{66}', '\u{67}', '\u{68}', '\u{69}', '\u{6A}', '\u{6B}', '\u{6C}', '\u{6D}', '\u{6E}', '\u{6F}',
                '\u{70}', '\u{71}', '\u{72}', '\u{73}', '\u{74}', '\u{75}', '\u{76}', '\u{77}', '\u{78}', '\u{79}', '\u{7A}', '\u{7B}', '\u{7C}', '\u{7D}', '\u{7E}', '\u{7F}'
            ]
        }
    }
    
    pub fn japanese_hiragana() -> SaikoCharacterSet {
        SaikoCharacterSet {
            characters : ('\u{3040}'..'\u{309f}').into_iter().collect()
        }
    }
    
    // pub const EXTENDED_ASCII : SaikoCharacterSet = SaikoCharacterSet {
    //     characters : vec![
    //         '\u{00}', '\u{01}', '\u{02}', '\u{03}', '\u{04}', '\u{05}', '\u{06}', '\u{07}', '\u{08}', '\u{09}', '\u{0A}', '\u{0B}', '\u{0C}', '\u{0D}', '\u{0E}', '\u{0F}',
    //         '\u{10}', '\u{11}', '\u{12}', '\u{13}', '\u{14}', '\u{15}', '\u{16}', '\u{17}', '\u{18}', '\u{19}', '\u{1A}', '\u{1B}', '\u{1C}', '\u{1D}', '\u{1E}', '\u{1F}',
    //         '\u{20}', '\u{21}', '\u{22}', '\u{23}', '\u{24}', '\u{25}', '\u{26}', '\u{27}', '\u{28}', '\u{29}', '\u{2A}', '\u{2B}', '\u{2C}', '\u{2D}', '\u{2E}', '\u{2F}',
    //         '\u{30}', '\u{31}', '\u{32}', '\u{33}', '\u{34}', '\u{35}', '\u{36}', '\u{37}', '\u{38}', '\u{39}', '\u{3A}', '\u{3B}', '\u{3C}', '\u{3D}', '\u{3E}', '\u{3F}',
    //         '\u{40}', '\u{41}', '\u{42}', '\u{43}', '\u{44}', '\u{45}', '\u{46}', '\u{47}', '\u{48}', '\u{49}', '\u{4A}', '\u{4B}', '\u{4C}', '\u{4D}', '\u{4E}', '\u{4F}',
    //         '\u{50}', '\u{51}', '\u{52}', '\u{53}', '\u{54}', '\u{55}', '\u{56}', '\u{57}', '\u{58}', '\u{59}', '\u{5A}', '\u{5B}', '\u{5C}', '\u{5D}', '\u{5E}', '\u{5F}',
    //         '\u{60}', '\u{61}', '\u{62}', '\u{63}', '\u{64}', '\u{65}', '\u{66}', '\u{67}', '\u{68}', '\u{69}', '\u{6A}', '\u{6B}', '\u{6C}', '\u{6D}', '\u{6E}', '\u{6F}',
    //         '\u{70}', '\u{71}', '\u{72}', '\u{73}', '\u{74}', '\u{75}', '\u{76}', '\u{77}', '\u{78}', '\u{79}', '\u{7A}', '\u{7B}', '\u{7C}', '\u{7D}', '\u{7E}', '\u{7F}',
    //         '\u{80}', '\u{81}', '\u{82}', '\u{83}', '\u{84}', '\u{85}', '\u{86}', '\u{87}', '\u{88}', '\u{89}', '\u{8A}', '\u{8B}', '\u{8C}', '\u{8D}', '\u{8E}', '\u{8F}',
    //         '\u{90}', '\u{91}', '\u{92}', '\u{93}', '\u{94}', '\u{95}', '\u{96}', '\u{97}', '\u{98}', '\u{99}', '\u{9A}', '\u{9B}', '\u{9C}', '\u{9D}', '\u{9E}', '\u{9F}',
    //         '\u{A0}', '\u{A1}', '\u{A2}', '\u{A3}', '\u{A4}', '\u{A5}', '\u{A6}', '\u{A7}', '\u{A8}', '\u{A9}', '\u{AA}', '\u{AB}', '\u{AC}', '\u{AD}', '\u{AE}', '\u{AF}',
    //         '\u{B0}', '\u{B1}', '\u{B2}', '\u{B3}', '\u{B4}', '\u{B5}', '\u{B6}', '\u{B7}', '\u{B8}', '\u{B9}', '\u{BA}', '\u{BB}', '\u{BC}', '\u{BD}', '\u{BE}', '\u{BF}',
    //         '\u{C0}', '\u{C1}', '\u{C2}', '\u{C3}', '\u{C4}', '\u{C5}', '\u{C6}', '\u{C7}', '\u{C8}', '\u{C9}', '\u{CA}', '\u{CB}', '\u{CC}', '\u{CD}', '\u{CE}', '\u{CF}',
    //         '\u{D0}', '\u{D1}', '\u{D2}', '\u{D3}', '\u{D4}', '\u{D5}', '\u{D6}', '\u{D7}', '\u{D8}', '\u{D9}', '\u{DA}', '\u{DB}', '\u{DC}', '\u{DD}', '\u{DE}', '\u{DF}',
    //         '\u{E0}', '\u{E1}', '\u{E2}', '\u{E3}', '\u{E4}', '\u{E5}', '\u{E6}', '\u{E7}', '\u{E8}', '\u{E9}', '\u{EA}', '\u{EB}', '\u{EC}', '\u{ED}', '\u{EE}', '\u{EF}',
    //         '\u{F0}', '\u{F1}', '\u{F2}', '\u{F3}', '\u{F4}', '\u{F5}', '\u{F6}', '\u{F7}', '\u{F8}', '\u{F9}', '\u{FA}', '\u{FB}', '\u{FC}', '\u{FD}', '\u{FE}', '\u{FF}',
    //     ]
    // };
    
    // pub const JAPANESE_HIRAGANA : SaikoCharacterSet = SaikoCharacterSet {
    //     characters : ('\u{3040}'..'\u{309f}').into_iter().collect()
    // };
    
    // pub const JAPANESE_KATAKANA : SaikoCharacterSet = SaikoCharacterSet {
    //     characters : ('\u{30a0}'..'\u{30ff}').into_iter().collect()
    // };
    
    // pub const JAPANESE_KANJI : SaikoCharacterSet = SaikoCharacterSet {
    //     characters : ('\u{4e00}'..'\u{9faf}').into_iter().collect()
    // };
    
    pub fn extend(&mut self, other : impl Into<SaikoCharacterSet>) {
        self.characters.extend(other.into().characters);
    }
}

impl IntoIterator for SaikoCharacterSet {
    type Item = char;
    type IntoIter = std::vec::IntoIter<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.characters.into_iter()
    }
}