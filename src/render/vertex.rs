use bevy::render::render_resource::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

//==============================================================================
//             SaikoRectInstance
//==============================================================================

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexRect {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub color: [f32; 4],
}

impl VertexRect {
    const ATTRIBUTES: [VertexFormat; 3] = [
        VertexFormat::Float32x3,
        VertexFormat::Float32x2,
        VertexFormat::Float32x4,
    ];
            
    pub fn desc() -> VertexBufferLayout {
        let attribs = Self::ATTRIBUTES.to_vec();
        VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, attribs)
    }
}