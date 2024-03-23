use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    prelude::*,
    render::{
        render_resource::{
            AsBindGroup, BindGroupLayout, BlendState, CachedRenderPipelineId, ColorTargetState,
            ColorWrites, FragmentState, MultisampleState, PipelineCache, PreparedBindGroup,
            PrimitiveState, RenderPipelineDescriptor, TextureFormat, TextureView,
        },
        renderer::RenderDevice,
        texture::{BevyDefault, FallbackImage},
    },
    utils::HashMap,
};

use super::{buffer::SaikoBuffer, SAIKO_SHADER_HANDLE};

//==============================================================================
//             SaikoRenderPipeline
//==============================================================================

#[derive(Resource)]
pub struct SaikoRenderPipeline {
    pub(crate) pipeline: CachedRenderPipelineId,
    pub(crate) bind_group_layout: BindGroupLayout,
    pub(crate) render_textures: HashMap<Entity, TextureView>,
    pub(crate) fallback_image: FallbackImage,
    pub(crate) bind_groups: Option<PreparedBindGroup<()>>,
}

impl FromWorld for SaikoRenderPipeline {
    fn from_world(world: &mut World) -> Self {
        let fallback_image = FallbackImage::from_world(world);
        let render_device = world.resource::<RenderDevice>();

        let bind_group_layout = SaikoBuffer::bind_group_layout(render_device);

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
            render_textures: HashMap::new(),
            fallback_image,
            bind_groups: None,
        }
    }
}
