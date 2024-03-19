use std::io::Read;

use bevy::{
    ecs::storage,
    math::{Vec2, Vec3, Vec4},
    render::render_resource::{
        AsBindGroup, BindingResource, Buffer, BufferBinding, IntoBinding, ShaderType,
        VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
    },
};

//==============================================================================
//             SaikoRectInstances
//==============================================================================

#[derive(AsBindGroup)]
pub struct SaikoBuffer {
    #[storage(0, read_only)]
    pub rectangles: Vec<RectBuffer>,
}

//==============================================================================
//             SaikoRectInstance
//==============================================================================

#[derive(ShaderType, Default)]
pub struct RectBuffer {
    #[size(16)]
    pub position: Vec3,
    pub size: Vec2,
    pub color: Vec4,
    pub corners: Vec4,
}

impl RectBuffer {
    pub fn with_position(mut self, position: impl Into<Vec3>) -> Self {
        self.position = position.into();
        self
    }

    pub fn with_size(mut self, size: impl Into<Vec2>) -> Self {
        self.size = size.into();
        self
    }

    pub fn with_color(mut self, color: impl Into<Vec4>) -> Self {
        self.color = color.into();
        self
    }

    pub fn with_corners(mut self, corners: impl Into<Vec4>) -> Self {
        self.corners = corners.into();
        self
    }
}

//==============================================================================
//             IntoStorageBuffer
//==============================================================================
