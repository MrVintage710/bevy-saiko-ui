

use bevy::{asset::{AssetLoader, AsyncReadExt}, ecs::system::CommandQueue, math::U16Vec2, prelude::*, render::{extract_resource::ExtractResource, render_resource::{Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension}, renderer::{RenderDevice, RenderQueue}, texture::GpuImage, Extract, RenderApp}, utils::{BoxedFuture, HashMap}};
use etagere::{euclid::{Box2D, UnknownUnit}, Allocation, AtlasAllocator, Size};
use thiserror::Error;
use ttf_parser::{Face, GlyphId};
use msdfgen::{Bitmap, FontExt, MsdfGeneratorConfig, Range, Rgb, Rgba};

use super::SaikoCharacterSet;


pub struct SaikoFontSdfPlugin;

impl Plugin for SaikoFontSdfPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset_loader::<SaikoFontSdfLoader>()
            .init_asset::<SaikoFontSdf>()
        ;
        
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        
        render_app
            .add_systems( ExtractSchedule, extract_sdf_fonts,)
        ;
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        
        render_app.init_resource::<SaikoGPUFontAtlas>();
    }
}

//==============================================================================
//             SaikoFont Systems
//==============================================================================

/// This system extracts fonts from the world and puts them into the correct font atlases
fn extract_sdf_fonts (
    mut gpu_font_atlas : ResMut<SaikoGPUFontAtlas>,
    fonts : Extract<Res<Assets<SaikoFontSdf>>>,
    render_queue : ResMut<RenderQueue>
) {
    for (id, font) in fonts.iter() {
        if gpu_font_atlas.layers.contains_key(&id) || !font.is_dirty {
            continue;
        }
        
        let layer = if let Some(layer) = gpu_font_atlas.layers.get(&id) { *layer } else {
            let layer = gpu_font_atlas.next_layer;
            gpu_font_atlas.next_layer += 1;
            layer
        };
        
        render_queue.write_texture(
            ImageCopyTexture {
                texture: &gpu_font_atlas.texture,
                mip_level: 0,
                origin: Origin3d { x: 0, y: 0, z: layer },
                aspect: TextureAspect::All,
            }, 
            font.raw_data(), 
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(FONT_ATLAS_DIMS * 16),
                rows_per_image: Some(FONT_ATLAS_DIMS),
            }, 
            Extent3d { width: FONT_ATLAS_DIMS, height: FONT_ATLAS_DIMS, depth_or_array_layers: 1 }
        );
        
        gpu_font_atlas.layers.insert(id, layer);
    }
}

fn initialize_sdf_font_texture (
    mut commands : Commands,
    render_device : Res<RenderDevice>
) {
    
}

//==============================================================================
//             SaikoFontArray
//==============================================================================

const FONT_ATLAS_DIMS : u32 = 2048;

#[derive(Asset, TypePath, Clone)]
pub struct SaikoFontSdf {
    bitmap : Vec<Rgba<f32>>,
    allocator : AtlasAllocator,
    glyph_data : HashMap<char, SaikoGlyphData>,
    glyph_size : u32,
    is_dirty : bool,
}

impl Default for SaikoFontSdf {
    fn default() -> Self {
        let bitmap = vec![Rgba::new(0.0, 0.0, 0.0, 1.0); FONT_ATLAS_DIMS as usize * FONT_ATLAS_DIMS as usize];
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
                target_pixel.r = source_pixel.r;
                target_pixel.g = source_pixel.g;
                target_pixel.b = source_pixel.b;
            }
        }
    }
    
    pub fn pixel(&self, x : u32, y : u32) -> &Rgba<f32> {
        &self.bitmap[((y*FONT_ATLAS_DIMS) + x) as usize]
    }
    
    fn pixel_mut(&mut self, x : u32, y : u32) -> &mut Rgba<f32> {
        self.is_dirty = true;
        &mut self.bitmap[((y*FONT_ATLAS_DIMS) + x) as usize]
    }
    
    pub fn pixels(&self) -> &[Rgba<f32>] {
        self.bitmap.as_slice()
    }
    
    pub fn raw_data(&self) -> &[u8] {
        let pixels = self.pixels();
        
        unsafe {
            core::slice::from_raw_parts(
                pixels.as_ptr() as _,
                pixels.len() * core::mem::size_of::<Rgba<f32>>(),
            )
        }
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
//             SaikoGPUFontAtlas
//==============================================================================

#[derive(Resource)]
pub struct SaikoGPUFontAtlas {
    texture : Texture,
    texture_view : TextureView,
    layers : HashMap<AssetId<SaikoFontSdf>, u32>,
    next_layer : u32,
}

impl SaikoGPUFontAtlas {
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn texture_view(&self) -> &TextureView {
        &self.texture_view
    }
}

impl FromWorld for SaikoGPUFontAtlas {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        
        let font_texture = render_device.create_texture(
            &TextureDescriptor { 
                label: Some("SaikoFontSdfAtlas"), 
                size: Extent3d {
                    width: FONT_ATLAS_DIMS,
                    height: FONT_ATLAS_DIMS,
                    depth_or_array_layers: 1,
                }, 
                mip_level_count: 1, 
                sample_count: 1, 
                dimension: TextureDimension::D2, 
                format: TextureFormat::Rgba32Float, 
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                view_formats: &[] 
            }
        );
        
        let texture_view = font_texture.create_view(&TextureViewDescriptor {
            label: Some("Saiko Font Atlas Texture View"),
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
            // format: Some(TextureFormat::Rgba32Float),
            // dimension: Some(TextureViewDimension::D2Array),
            // aspect:TextureAspect::All,
            // base_mip_level: 0,
            // mip_level_count: Some(1),
            // base_array_layer: todo!(),
            // array_layer_count: todo!(),
        });
        
        Self { texture : font_texture, texture_view, layers : HashMap::new(), next_layer : 0 }
    }
}
