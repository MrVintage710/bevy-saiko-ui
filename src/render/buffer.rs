use bevy::render::render_resource::{AsBindGroup, BindingResource, Buffer, BufferBinding, IntoBinding, ShaderType, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

//==============================================================================
//             SaikoRectInstance
//==============================================================================

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct BufferRect {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub color: [f32; 4],
    pub corners: [f32; 4],
    pub _padding: [f32; 3],
}

impl BufferRect {
    pub const SIZE : usize = std::mem::size_of::<Self>();
    
    const ATTRIBUTES: [VertexFormat; 5] = [
        VertexFormat::Float32x3,
        VertexFormat::Float32x2,
        VertexFormat::Float32x4,
        VertexFormat::Float32x4,
        VertexFormat::Float32x3,
    ];
            
    pub fn desc() -> VertexBufferLayout {
        let attribs = Self::ATTRIBUTES.to_vec();
        VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, attribs)
    }
    
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}