use bevy::{
    ecs::storage,
    math::{Vec2, Vec3, Vec4},
    prelude::*,
    render::{render_resource::{AsBindGroup, PreparedBindGroup, ShaderType, Texture, TextureId, TextureView}, texture},
};

use crate::common::bounds::Bounds;

//==============================================================================
//             Saikobuffer
//==============================================================================

#[derive(AsBindGroup, Default)]
pub struct SaikoBuffer {
    #[storage(0, read_only)]
    pub rectangles: Vec<RectBuffer>,
    // #[texture(1, dimension = "2d_array", visibility(fragment))]
    // #[sampler(2)]
    // pub fonts : Handle<Image>,
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
pub struct SaikoPreparedBuffer(pub PreparedBindGroup<()>);

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
}

//==============================================================================
//             FontIntoBuffer
//==============================================================================

// #[derive(ShaderType)]
// pub struct FontAtlasBuffer {
//     fonts : TextureView
// }

// #[derive(ShaderType)]
// pub struct FontAtlasFamilyBuffer {
//     glyphs : Vec<Vec2>,
//     uv_delta : f32,
//     layer : u32,
// }

// #[derive(ShaderType)]
// pub struct FontAtlasFamilyBuffer {
//     glyphs : Vec<>
// }

#[derive(ShaderType)]
pub struct FontAtlasGlyphBuffer {
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