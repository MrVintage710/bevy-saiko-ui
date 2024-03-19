use std::num::NonZeroU64;

use bevy::{core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state, prelude::*, render::{extract_component::ExtractComponent, mesh::PrimitiveTopology, render_resource::{BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Buffer, BufferBinding, BufferBindingType, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, FrontFace, MultisampleState, PipelineCache, PolygonMode, PrimitiveState, RenderPipelineDescriptor, ShaderDefVal, ShaderStages, ShaderType, SpecializedRenderPipeline, TextureFormat, TextureView, VertexState}, renderer::RenderDevice, texture::BevyDefault}, utils::HashMap};

use super::{buffer::BufferRect, SAIKO_SHADER_HANDLE};

//==============================================================================
//             SaikoRenderPipeline
//==============================================================================

#[derive(Resource)]
pub struct SaikoRenderPipeline {
    pub(crate) pipeline : CachedRenderPipelineId,
    pub(crate) bind_group_layout: BindGroupLayout,
    pub(crate) render_texture : HashMap<Entity, TextureView>,
}

impl FromWorld for SaikoRenderPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        
        let bind_group_layout = render_device.create_bind_group_layout(
            "SaikoUI BindGroupLayout", 
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer { 
                        ty: BufferBindingType::Storage { read_only: true }, 
                        has_dynamic_offset: false, 
                        min_binding_size: NonZeroU64::new(BufferRect::SIZE as u64)
                    },
                    count: None,
                }
            ]
        );
        
        let pipeline = RenderPipelineDescriptor {
            label: Some("SaikoUI Render Pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            // layout: vec![],
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader: SAIKO_SHADER_HANDLE,
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            multisample: MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            push_constant_ranges: vec![],
            depth_stencil: None,
        };
        
        let pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(pipeline);
        
        //This is where I would create bind group layouts if I had them
        SaikoRenderPipeline { 
            pipeline,
            bind_group_layout,
            render_texture: HashMap::new(),
        }
    }
}

//==============================================================================
//             SaikoShaderData
//==============================================================================

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct SaikoShaderData {
    pub test : f32,
}

