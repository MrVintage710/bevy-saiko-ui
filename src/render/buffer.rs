use std::num::NonZeroU32;

use bevy::{
    math::{Vec2, Vec4},
    prelude::*,
    render::{render_asset::RenderAssets, render_resource::{AsBindGroup, AsBindGroupError, BindGroup, BindGroupLayout, BindGroupLayoutEntry, BindingType, PreparedBindGroup, SamplerBindingType, ShaderStages, ShaderType, Texture, TextureView, TextureViewDimension, UnpreparedBindGroup}, renderer::RenderDevice, texture::FallbackImage}, utils::HashMap,
};

use crate::common::bounds::Bounds;


pub trait ManualShaderType {
    fn bind_group_layout(render_device : &RenderDevice) -> BindGroupLayout;
}

//==============================================================================
//             Saikobuffer
//==============================================================================

#[derive(AsBindGroup, Default)]
pub struct SaikoBuffer {
    #[storage(0, read_only)]
    pub rectangles: Vec<RectBuffer>,
    // #[storage(1, read_only)]
    // pub text_glyphs: Vec<TextGlyphBuffer>,
    #[uniform(1)]
    pub screen_size: Vec2,
}

impl SaikoBuffer {
    pub const NUMBER_OF_ENTRIES: u32 = 2;
    
    pub fn push_rect(&mut self, rect: impl Into<RectBuffer>) {
        self.rectangles.push(rect.into())
    }
}

//==============================================================================
//             SaikoPreparedBuffer
//==============================================================================

#[derive(Component)]
pub struct SaikoPreparedBuffer(pub PreparedBindGroup<()>, pub BindGroup);

//==============================================================================
//             RectBuffer
//==============================================================================

#[derive(ShaderType, Default)]
pub struct RectBuffer {
    pub bound : Bounds,
    pub border_style: BorderStyleBuffer,
    pub fill_style: FillStyleBuffer,
}

impl RectBuffer {
    pub fn with_position(mut self, position: impl Into<Vec2>) -> Self {
        self.bound.center = position.into();
        self
    }

    pub fn with_size(mut self, size: impl Into<Vec2>) -> Self {
        self.bound.size = size.into();
        self
    }
    
    pub fn with_color(mut self, color : impl Into<Color>) -> Self {
        self.fill_style.fill_color = color.into();
        self
    }
    
    pub fn with_border_color(mut self, color : impl Into<Color>) -> Self {
        self.border_style.border_color = color.into();
        self
    }
    
    pub fn with_border_radius(mut self, radius : impl Into<Vec4>) -> Self {
        self.border_style.border_radius = radius.into();
        self
    }
    
    pub fn with_border_width(mut self, width : f32) -> Self {
        self.border_style.border_width = width;
        self
    }
}

//==============================================================================
//             ImageBuffer
//==============================================================================

#[derive(ShaderType)]
pub struct GlyphBuffer {
    bound : Bounds,
    family : u32,
    uv : Vec2,
}

//==============================================================================
//             FontIntoBuffer
//==============================================================================

// pub struct FontAtlasSdfBuffer;

// impl FontAtlasSdfBuffer {
//     pub fn bind_group_layout(render_device : &RenderDevice) -> BindGroupLayout {
//         render_device.create_bind_group_layout(
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
//                     count: Some(NonZeroU32::new().unwrap()),
//                 },
//                 BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
//                     count: None,
//                 },
//             ]
//         )
//     }
// }

#[derive(ShaderType)]
pub struct TextGlyphBuffer {
    pub bound : Bounds,
    pub family : u32,
    pub uv_min : Vec2,
    pub uv_dim : f32,
}

//==============================================================================
//             BorderStyleBuffer
//==============================================================================

#[derive(ShaderType, Clone, Copy)]
pub struct BorderStyleBuffer {
    pub border_color: Color,
    pub border_radius: Vec4,
    pub border_width: f32,
}

impl Default for BorderStyleBuffer {
    fn default() -> Self {
        BorderStyleBuffer {
            border_color: Color::BLACK,
            border_radius: Vec4::ZERO,
            border_width: 5.0,
        }
    }
}

//==============================================================================
//             FillStyleBuffer
//==============================================================================

#[derive(ShaderType, Clone, Copy)]
pub struct FillStyleBuffer {
    pub fill_color: Color,
}

impl Default for FillStyleBuffer {
    fn default() -> Self {
        FillStyleBuffer {
            fill_color: Color::WHITE,
        }
    }
}