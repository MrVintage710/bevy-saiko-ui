use std::num::NonZeroU32;

use bevy::{asset::{AssetLoader, AsyncReadExt}, ecs::reflect, math::U16Vec2, prelude::*, render::{render_resource::{AddressMode, BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension}, renderer::{RenderDevice, RenderQueue}, Extract, RenderApp}, utils::{BoxedFuture, HashMap}};
use etagere::{euclid::{Box2D, UnknownUnit}, Allocation, AtlasAllocator, Size};
use fit_text::CharacterWidthCache;
use thiserror::Error;
use ttf_parser::{Face, GlyphId};
use msdfgen::{Bitmap, FillRule, FontExt, MsdfGeneratorConfig, Range, Rgb, Rgba};

use crate::{common::bounds::Bounds, render::pipeline::SaikoRenderPipeline};

use super::SaikoCharacterSet;

pub const DEFAULT_FONT : Handle<SaikoFontSdf> = Handle::weak_from_u128(11068737277721006659);

pub struct SaikoFontSdfPlugin;

impl Plugin for SaikoFontSdfPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(First, reset_fonts)
            
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
        //TODO: Should be relativly fast but could studder the loading. Check for this later and handle accordingling.
        //This loads and inits the default font for the application.
        let font = Face::parse(notosans::REGULAR_TTF, 0).expect("SaikoUi : There was an error parsing the default font.");
        let mut font_atlas = SaikoFontSdf::default();
        font_atlas.add_glyphs(&font, SaikoCharacterSet::ascii());
        app.world.get_resource_mut::<Assets<SaikoFontSdf>>().unwrap().insert(DEFAULT_FONT, font_atlas.clone());
        
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        
        let render_device = render_app.world.get_resource::<RenderDevice>().unwrap();
        let render_queue = render_app.world.get_resource::<RenderQueue>().unwrap();
        
        let gpu_fonts = GpuSaikoFonts::new(render_device, render_queue, font_atlas);
        render_app.insert_resource(gpu_fonts);
        
        // render_app.init_resource::<GpuSaikoFonts>();
    }
}

//==============================================================================
//             SaikoFont Systems
//==============================================================================

/// This system extracts fonts from the world and puts them into the correct font atlases
fn extract_sdf_fonts (
    mut gpu_fonts : ResMut<GpuSaikoFonts>,
    fonts : Extract<Res<Assets<SaikoFontSdf>>>,
    render_queue : Res<RenderQueue>,
    render_device : Res<RenderDevice>
) {
    for (id, font) in fonts.iter() {
        if gpu_fonts.contains(&id) || !font.is_dirty {
            continue;
        }
        
        let gpu_font = gpu_fonts.get_or_insert(id, render_device.as_ref());
        if font.is_dirty {
            font.write_to_texture(render_queue.as_ref(), &gpu_font.texture);
        }
    }
}

fn reset_fonts(
    mut fonts : ResMut<Assets<SaikoFontSdf>>
) {
    for font in fonts.iter_mut() {
        font.1.is_dirty = false;
    }
}

//==============================================================================
//             SaikoFontArray
//==============================================================================

const FONT_ATLAS_DIMS : u32 = 2048;

#[derive(Asset, Reflect, Clone)]
#[reflect(Default)]
pub struct SaikoFontSdf {
    #[reflect(ignore)]
    bitmap : Vec<Rgba<f32>>,
    #[reflect(ignore)]
    allocator : AtlasAllocator,
    #[reflect(ignore)]
    glyph_data : HashMap<char, GlyphMetrics>,
    glyph_size : u32,
    is_dirty : bool,
}

impl Default for SaikoFontSdf {
    fn default() -> Self {
        let bitmap = vec![Rgba::new(0.0, 0.0, 0.0, 1.0); FONT_ATLAS_DIMS as usize * FONT_ATLAS_DIMS as usize];
        let allocator = AtlasAllocator::new(Size::splat(FONT_ATLAS_DIMS as i32));
        let glyph_size = 40;
        
        Self { bitmap, allocator, glyph_size, glyph_data : HashMap::default(), is_dirty : true }
    }
}

impl CharacterWidthCache for SaikoFontSdf {
    fn char_width(&mut self, character: char, font_size: u32) -> f64 {
        if character == ' ' { return font_size as f64 * 0.1; }
        self.glyph_data.get(&character).map_or(0.0, |glyph| glyph.h_advance(font_size))
    }
}

impl SaikoFontSdf {
    pub fn add_glyphs(&mut self, font : &Face, characters : impl IntoIterator<Item = char>) {
        for character in characters.into_iter() {
            if self.glyph_data.contains_key(&character) { continue; }
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
        
        glyph_shape.edge_coloring_simple(16.0, 0);
        let config = MsdfGeneratorConfig::default();
        glyph_shape.generate_msdf(&mut bitmap, framing, config);
        glyph_shape.correct_sign(&mut bitmap, &framing, FillRule::default());
        glyph_shape.correct_msdf_error(&mut bitmap, framing, config);
        bitmap.flip_y();
        
        self.insert_bitmap(bitmap, allocation);
        
        true
    }
    
    pub fn metrics(&self, character : char) -> Option<&GlyphMetrics> {
        self.glyph_data.get(&character)
    }
    
    pub fn metrics_map(&self, characters : impl IntoIterator<Item = char>) -> HashMap<char, GlyphMetrics> {
        let mut map = HashMap::new();
        
        for character in characters.into_iter() {
            if map.contains_key(&character) { continue; }
            match character {
                ' ' => { map.insert(character, GlyphMetrics::space(0.1)); }
                _ => {
                    if let Some(metrics) = self.metrics(character) {
                        map.insert(character, metrics.clone());
                    }
                }
            }
        }
        
        map
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
        
        self.is_dirty = true;
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
        let Some(allocation) = self.allocator.allocate(Size::splat(self.glyph_size as i32)) else { return None };
        let rect = self.allocator.get(allocation.id);
        //Save Glyph Metrics
        let glyph_metrics = GlyphMetrics::from_glyph(glyph_id, font, rect);
        self.glyph_data.insert(character, glyph_metrics);
        Some(allocation)
    }
    
    fn write_to_texture(&self, render_queue : &RenderQueue, texture : &Texture) {
        render_queue.write_texture(
            ImageCopyTexture {
                texture,
                mip_level: 0,
                origin: Origin3d { x: 0, y: 0, z: 0},
                aspect: TextureAspect::All,
            }, 
            self.raw_data(), 
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(FONT_ATLAS_DIMS * 16),
                rows_per_image: Some(FONT_ATLAS_DIMS),
            }, 
            Extent3d { width: FONT_ATLAS_DIMS, height: FONT_ATLAS_DIMS, depth_or_array_layers: 1 }
        );
    }
    
    // pub fn shape(&self, text : String)
}

//==============================================================================
//             SaikoGlyphData
//==============================================================================

#[derive(Clone, Reflect, Debug)]
pub struct GlyphMetrics {
    width : f64,
    height : f64,
    x : f64,
    y : f64,
    h_advance : f64,
    v_advance : f64,
    #[reflect(ignore)]
    atlas_location : Option<Box2D<i32, UnknownUnit>>,
}

impl GlyphMetrics {
    pub fn space(space : f64) -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            x: 0.0,
            y: 0.0,
            h_advance: space,
            v_advance: 0.0,
            atlas_location: None,
        }
    }
    
    pub fn from_glyph(glyph : GlyphId, font : &Face, area : Box2D<i32, UnknownUnit>) -> Self {
        let font_height = font.height() as f64;
        let h_advance = (font.glyph_hor_advance(glyph).unwrap_or(0) as f64) / font_height;
        let v_advance = (font.glyph_ver_advance(glyph).unwrap_or(0) as f64) / font_height;
        let rect = font.glyph_bounding_box(glyph).unwrap();
        let width = rect.width() as f64 / font_height;
        let height = rect.height() as f64 / font_height;
        let x = rect.x_min as f64 / font_height;
        let y = rect.y_min as f64 / font_height;
        let y_origin = font.glyph_y_origin(glyph).unwrap_or(0) as f64 / font_height;
        
        Self { width, height, x, y : y + y_origin, h_advance, v_advance, atlas_location: Some(area) }
    }
    
    pub fn width(&self, target_height : u32) -> f64 {
        self.width * (target_height as f64)
    }
    
    pub fn height(&self, target_height : u32) -> f64 {
        self.height * (target_height as f64)
    }
    
    pub fn x(&self, target_height : u32) -> f64 {
        self.x * (target_height as f64)
    }
    
    pub fn y(&self, target_height : u32) -> f64 {
        self.y * (target_height as f64)
    }
    
    pub fn h_advance(&self, target_height : u32) -> f64 {
        self.h_advance * (target_height as f64)
    }
    
    pub fn v_advance(&self, target_height : u32) -> f64 {
        self.v_advance * (target_height as f64)
    }
    
    pub fn atlas_location(&self) -> Option<Box2D<i32, UnknownUnit>> {
        self.atlas_location
    }
}

//==============================================================================
//             GpuSaikoFontSdf
//==============================================================================

pub struct GpuSaikoFontSdf {
    pub(crate) texture : Texture,
    pub(crate) texture_view : TextureView,
}

//==============================================================================
//             GpuSaikoFonts
//==============================================================================

#[derive(Resource)]
pub struct GpuSaikoFonts {
    default_sdf_font : GpuSaikoFontSdf,
    fonts : HashMap<AssetId<SaikoFontSdf>, GpuSaikoFontSdf>,
}

impl GpuSaikoFonts {
    pub const MAX_FONTS : u32 = 32;
    
    pub fn new(render_device : &RenderDevice, render_queue : &RenderQueue, default_font : SaikoFontSdf) -> Self {
        
        let (texture, texture_view) = GpuSaikoFonts::generate_texture(render_device);
        default_font.write_to_texture(render_queue, &texture);
        
        Self {
            default_sdf_font: GpuSaikoFontSdf { texture, texture_view},
            fonts: HashMap::new(),
        }
    }
    
    ///This function will get the font texture if it exitsts, or create it if it doesn't.
    ///It will not load in texture data.
    pub fn get_or_insert(&mut self, handle : AssetId<SaikoFontSdf>, render_device : &RenderDevice) -> &GpuSaikoFontSdf {
        self.fonts.entry(handle).or_insert_with(|| {
            let (texture, texture_view) = GpuSaikoFonts::generate_texture(render_device);
            GpuSaikoFontSdf { texture, texture_view }
        })
    }
    
    pub fn contains(&self, handle : &AssetId<SaikoFontSdf>) -> bool {
        self.fonts.contains_key(handle)
    }
    
    fn generate_texture(render_device : &RenderDevice) -> (Texture, TextureView) {
        let texture = render_device.create_texture(
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
        
        let texture_view = texture.create_view(&TextureViewDescriptor {
            label: Some("Saiko Font Atlas Texture View"),
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
        });
        
        (texture, texture_view)
    }
    
    pub fn create_master_bind(&self, render_device : &RenderDevice, pipeline : &SaikoRenderPipeline) -> BindGroup {
        //Dereferencing the texture views to get the wpgu texture views
        let mut fonts = vec![&(*self.default_sdf_font.texture_view)];
        for (_, font) in &self.fonts {
            fonts.push(&(*font.texture_view));
        }
        
        while fonts.len() < 32 {
            fonts.push(&(*self.default_sdf_font.texture_view));
        }
        
        render_device.create_bind_group(
            "Saiko Font Bind Group", 
            &pipeline.font_bind_group_layout, 
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(fonts.as_slice()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&pipeline.font_sampler),
                },
            ]
        )
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
