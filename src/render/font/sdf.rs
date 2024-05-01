

use std::num::NonZeroU32;

use bevy::{asset::{AssetLoader, AsyncReadExt}, ecs::reflect, math::U16Vec2, prelude::*, render::{render_resource::{AddressMode, BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension}, renderer::{RenderDevice, RenderQueue}, Extract, RenderApp}, utils::{BoxedFuture, HashMap}};
use etagere::{euclid::{Box2D, UnknownUnit}, Allocation, AtlasAllocator, Size};
use thiserror::Error;
use ttf_parser::{Face, GlyphId};
use msdfgen::{Bitmap, FillRule, FontExt, MsdfGeneratorConfig, Range, Rgb, Rgba};

use crate::render::pipeline::SaikoRenderPipeline;

use super::SaikoCharacterSet;


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
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        
        // render_app.init_resource::<GpuSaikoFonts>();
    }
}

//==============================================================================
//             SaikoFont Systems
//==============================================================================

/// This system extracts fonts from the world and puts them into the correct font atlases
fn extract_sdf_fonts (
    mut gpu_fonts : ResMut<SaikoRenderPipeline>,
    fonts : Extract<Res<Assets<SaikoFontSdf>>>,
    render_queue : Res<RenderQueue>,
    render_device : Res<RenderDevice>
) {
    for (id, font) in fonts.iter() {
        if gpu_fonts.fonts.contains(&id) || !font.is_dirty {
            continue;
        }
        
        let gpu_font = gpu_fonts.fonts.get_or_insert(id, render_device.as_ref());
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

#[derive(Asset, Reflect)]
#[reflect(Default)]
pub struct SaikoFontSdf {
    #[reflect(ignore)]
    bitmap : Vec<Rgba<f32>>,
    #[reflect(ignore)]
    allocator : AtlasAllocator,
    #[reflect(ignore)]
    glyph_data : HashMap<char, SaikoGlyphData>,
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
        
        let data = SaikoGlyphData::from_font(glyph_id, font, allocation, rect);
        self.glyph_data.insert(character, data);
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
//             GpuSaikoFontSdf
//==============================================================================

pub struct GpuSaikoFontSdf {
    pub(crate) texture : Texture,
    pub(crate) texture_view : TextureView,
}

//==============================================================================
//             GpuSaikoFonts
//==============================================================================

pub struct GpuSaikoFonts {
    default_sdf_font : GpuSaikoFontSdf,
    fonts : HashMap<AssetId<SaikoFontSdf>, GpuSaikoFontSdf>,
    sampler : Sampler,
    bind_group_layout : BindGroupLayout,
}

impl GpuSaikoFonts {
    pub const MAX_FONTS : u32 = 32;
    
    pub fn new(render_device : &RenderDevice, render_queue : &RenderQueue) -> Self {
        let (texture, texture_view) = GpuSaikoFonts::generate_texture(render_device);
        
        let Ok(font) = Face::parse(notosans::REGULAR_TTF, 0) else { panic!("Failed to load default font") };
        let mut font_atlas = SaikoFontSdf::default();
        
        font_atlas.add_glyphs(&font, SaikoCharacterSet::ascii());
        font_atlas.write_to_texture(render_queue, &texture);
        
        let font_atlas_sampler = render_device.create_sampler(&SamplerDescriptor {
            label: Some("Saiko SDF Font Atlas Sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            // mipmap_filter: todo!(),
            // lod_min_clamp: todo!(),
            // lod_max_clamp: todo!(),
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
            ..Default::default()
        });
        
        let bind_group_layout = render_device.create_bind_group_layout(
            "SaikoFontAtlasSdf",
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture { 
                        sample_type: bevy::render::render_resource::TextureSampleType::Float { filterable: true }, 
                        view_dimension: TextureViewDimension::D2Array, 
                        multisampled: false 
                    },
                    count: Some(NonZeroU32::new(GpuSaikoFonts::MAX_FONTS).unwrap()),
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ]
        );
        
        Self {
            default_sdf_font: GpuSaikoFontSdf { texture, texture_view},
            sampler: font_atlas_sampler,
            fonts: HashMap::new(),
            bind_group_layout,
        }
    }
    
    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
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
    
    pub fn create_master_bind(&self, render_device : &RenderDevice) -> BindGroup {
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
            &self.bind_group_layout, 
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(fonts.as_slice()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&self.sampler),
                },
            ]
        )
    }
}

impl FromWorld for GpuSaikoFonts {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let render_queue = world.get_resource::<RenderQueue>().unwrap();
        Self::new(render_device, render_queue)
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

// #[derive(Resource)]
// pub struct SaikoGPUFontAtlas {
//     // texture : Texture,
//     // texture_view : TextureView,
//     bind_group : BindGroupLayout,
//     sampler : Sampler,
//     layers : HashMap<AssetId<SaikoFontSdf>, u32>,
//     length : u32,
//     available : Vec<u32>,
//     is_dirty : bool,
// }

// impl SaikoGPUFontAtlas {
//     pub const MAX_LAYERS : u32 = 128;
    
//     // pub fn texture(&self) -> &Texture {
//     //     &self.texture
//     // }

//     // pub fn texture_view(&self) -> &TextureView {
//     //     &self.texture_view
//     // }
    
//     pub fn create_bind_group(&self, render_device : &RenderDevice, textures : BindingResource) -> BindGroup {
//         render_device.create_bind_group(
//             "SaikoBindGroup", 
//             &self.bind_group, 
//             &[
//                 BindGroupEntry {
//                     binding: 0,
//                     resource: textures,
//                 },
//                 BindGroupEntry {
//                     binding: 1,
//                     resource: BindingResource::Sampler(&self.sampler),
//                 },
//             ]
//         )
//     }
    
//     pub fn contians(&self, font_id : &Handle<SaikoFontSdf>) -> bool {
//         self.layers.contains_key(&font_id.id())
//     }
    
//     pub fn check_and_insert(&mut self, font_id : &Handle<SaikoFontSdf>) {
//         if self.contians(font_id) { return }
//         self.layers.insert(font_id.id(), self.length);
//         self.length += 1;
//         self.is_dirty = true;
//     }
    
//     pub fn sync(&mut self, query : &Extract<Res<Assets<SaikoFontSdf>>>, render_queue : &RenderQueue) {
//         let new_fonts = query.iter()
//             .filter(|item| !self.layers.contains_key(&item.0))
//             .map(|item| item.1)
//             .collect::<Vec<_>>();
        
//         let missing_fonts = self.layers.iter()
//             .filter(|a| !query.iter().any(|b| a.0 == &b.0))
//             // .map(|item| item.1)
//             .collect::<Vec<_>>();
        
//         if new_fonts.is_empty() && missing_fonts.is_empty() { return }
        
//         let layer_dif = new_fonts.len() as i32 - missing_fonts.len() as i32;
        
        
//     }
    
//     fn add_font(&mut self, font : &SaikoFontSdf, render_queue : &RenderQueue) {
//         // render_queue.write_texture(
//         //     ImageCopyTexture {
//         //         texture: &self.texture,
//         //         mip_level: 0,
//         //         origin: Origin3d { x: 0, y: 0, z: layer },
//         //         aspect: TextureAspect::All,
//         //     }, 
//         //     font.raw_data(), 
//         //     ImageDataLayout {
//         //         offset: 0,
//         //         bytes_per_row: Some(FONT_ATLAS_DIMS * 16),
//         //         rows_per_image: Some(FONT_ATLAS_DIMS),
//         //     }, 
//         //     Extent3d { width: FONT_ATLAS_DIMS, height: FONT_ATLAS_DIMS, depth_or_array_layers: 1 }
//         // );
//     }
    
//     fn remove_font(&mut self, layer : (AssetId<SaikoFontSdf>, u32)) {
//         self.layers.remove(&layer.0);
//         self.available.push(layer.1);
//     }
    
//     fn recompute_bind_group(&mut self, render_device : &RenderDevice) {
//         self.bind_group = render_device.create_bind_group_layout(
//             "SaikoFontAtlasSdf",
//             &[
//                 BindGroupLayoutEntry {
//                     binding: 0,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Texture { 
//                         sample_type: bevy::render::render_resource::TextureSampleType::Float { filterable: false }, 
//                         view_dimension: TextureViewDimension::D2Array, 
//                         multisampled: false 
//                     },
//                     count: Some(NonZeroU32::new(self.length).unwrap()),
//                 },
//                 BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
//                     count: None,
//                 },
//             ]
//         );
//     }
// }

// impl FromWorld for SaikoGPUFontAtlas {
//     fn from_world(world: &mut World) -> Self {
//         let render_device = world.get_resource::<RenderDevice>().unwrap();
//         // let render_queue = world.get_resource::<RenderQueue>().unwrap();
        
//         let texture = render_device.create_texture(
//             &TextureDescriptor {
//                 label: Some("SaikoFontSdfAtlas"), 
//                 size: Extent3d {
//                     width: FONT_ATLAS_DIMS,
//                     height: FONT_ATLAS_DIMS,
//                     depth_or_array_layers: SaikoGPUFontAtlas::MAX_LAYERS,
//                 }, 
//                 mip_level_count: 1, 
//                 sample_count: 1, 
//                 dimension: TextureDimension::D2, 
//                 format: TextureFormat::Rgba32Float, 
//                 usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
//                 view_formats: &[] 
//             }
//         );
        
//         let texture_view = texture.create_view(&TextureViewDescriptor {
//             label: Some("Saiko Font Atlas Texture View"),
//             dimension: Some(TextureViewDimension::D2Array),
//             ..Default::default()
//             // format: Some(TextureFormat::Rgba32Float),
//             // dimension: Some(TextureViewDimension::D2Array),
//             // aspect:TextureAspect::All,
//             // base_mip_level: 0,
//             // mip_level_count: Some(1),
//             // base_array_layer: todo!(),
//             // array_layer_count: todo!(),
//         });
        
//         Self { 
//             texture, 
//             texture_view, 
//             layers : HashMap::new(), 
//             length : 0, 
//             available : Vec::new(), 
//             is_dirty : false 
//         }
//     }
// }
