use bevy::{prelude::*, render::{extract_component::ExtractComponent, mesh::PrimitiveTopology, render_resource::{CachedRenderPipelineId, FragmentState, FrontFace, MultisampleState, PipelineCache, PolygonMode, PrimitiveState, RenderPipelineDescriptor, ShaderType, SpecializedRenderPipeline, VertexState}}};

use super::{vertex::VertexRect, SAIKO_SHADER_HANDLE};

//==============================================================================
//             SaikoRenderPipeline
//==============================================================================

#[derive(Resource)]
pub struct SaikoRenderPipeline {
    pub(crate) pipeline : CachedRenderPipelineId,
    // bind_group_layout: BindGroupLayout,
}

impl FromWorld for SaikoRenderPipeline {
    fn from_world(world: &mut World) -> Self {
        let pipeline = RenderPipelineDescriptor {
            label: Some("SaikoUI Render Pipeline".into()),
            layout: vec![],
            push_constant_ranges: vec![],
            vertex: VertexState {
                shader: SAIKO_SHADER_HANDLE,
                shader_defs: vec![],
                entry_point: "vertex".into(),
                buffers: vec![VertexRect::desc()]
            },
            fragment: Some(FragmentState {
                shader: SAIKO_SHADER_HANDLE,
                shader_defs: vec![],
                entry_point: "frament".into(),
                targets: vec![],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::PointList,
                front_face: FrontFace::default(),
                polygon_mode: PolygonMode::Point,
                strip_index_format: None,
                cull_mode: None,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
        };
        
        let pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(pipeline);
        
        //This is where I would create bind group layouts if I had them
        SaikoRenderPipeline { 
            pipeline,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SaikoPipleingKey;

impl SpecializedRenderPipeline for SaikoRenderPipeline {
    type Key = SaikoPipleingKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("SaikoUI Render Pipeline".into()),
            layout: vec![],
            push_constant_ranges: vec![],
            vertex: VertexState {
                shader: SAIKO_SHADER_HANDLE,
                shader_defs: vec![],
                entry_point: "vertex".into(),
                buffers: vec![VertexRect::desc()]
            },
            fragment: Some(FragmentState {
                shader: SAIKO_SHADER_HANDLE,
                shader_defs: vec![],
                entry_point: "frament".into(),
                targets: vec![],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::PointList,
                front_face: FrontFace::default(),
                polygon_mode: PolygonMode::Point,
                strip_index_format: None,
                cull_mode: None,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
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

