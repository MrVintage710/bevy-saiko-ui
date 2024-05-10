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

#[derive(AsBindGroup, Default, Component)]
pub struct SaikoBuffer {
    #[storage(0, read_only)]
    pub rectangles: Vec<SimpleShapeBuffer>,
    #[storage(1, read_only)]
    pub circles: Vec<SimpleShapeBuffer>,
    #[storage(2, read_only)]
    pub lines: Vec<LineBuffer>,
    #[uniform(3)]
    pub screen_size: Vec2,
}

impl SaikoBuffer {
    // pub const NUMBER_OF_ENTRIES: u32 = 2;
    
    pub fn push_rect(&mut self, rect: impl Into<SimpleShapeBuffer>) {
        self.rectangles.push(rect.into())
    }
    
    pub fn push_circle(&mut self, circle: impl Into<SimpleShapeBuffer>) {
        self.circles.push(circle.into())
    }
    
    pub fn push_line(&mut self, line: impl Into<LineBuffer>) {
        self.lines.push(line.into())
    }
    
    pub fn append(&mut self, other: &Self) {
        self.rectangles.append(&mut other.rectangles.clone());
        self.circles.append(&mut other.circles.clone());
        self.lines.append(&mut other.lines.clone());
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

/// This is a buffer that describes any shape thato fits in a bounds and is styled with a fill and border.
#[derive(ShaderType, Default, Clone)]
pub struct SimpleShapeBuffer {
    pub bound : Bounds,
    pub border_style: BorderStyleBuffer,
    pub fill_style: FillStyleBuffer,
}

impl SimpleShapeBuffer {
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
//             RectBuffer
//==============================================================================

#[derive(ShaderType, Default, Clone)]
pub struct LineBuffer {
    pub bounds : Bounds,
    pub line_style: LineStyleBuffer,
    pub border_style: BorderStyleBuffer,
    pub fill_style: FillStyleBuffer,
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
pub struct GlyphBuffer {
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

//==============================================================================
//             LineStyleBuffer
//==============================================================================

#[derive(ShaderType, Clone, Copy)]
pub struct LineStyleBuffer {
    pub thickness: f32,
}

impl Default for LineStyleBuffer {
    fn default() -> Self {
        LineStyleBuffer { thickness: 1.0 }
    }
}