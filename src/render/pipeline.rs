use bevy::{
    core_pipeline::{
        blit::{BlitPipeline, BlitPipelineKey},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::system::SystemState,
    prelude::*,
    render::{
        render_resource::{
            binding_types::{sampler, texture_2d},
            AsBindGroup, BindGroupLayout, BindGroupLayoutEntries, BlendState,
            CachedRenderPipelineId, ColorTargetState, ColorWrites, Extent3d, FragmentState,
            MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor,
            SamplerBindingType, ShaderStages, SpecializedRenderPipelines, TextureDescriptor,
            TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView,
            TextureViewDescriptor,
        },
        renderer::RenderDevice,
        texture::{BevyDefault, FallbackImage},
        view::ViewTarget,
        Render, RenderApp, RenderSet,
    },
    utils::HashMap,
};

use super::{buffer::SaikoBuffer, BLIT_SHADER_HANDLE, SAIKO_SHADER_HANDLE};

//==============================================================================
//             RenderPipelinePlugin
//==============================================================================

pub struct SaikoRenderPipelinePlugin;

impl Plugin for SaikoRenderPipelinePlugin {
    fn build(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(
            Render,
            update_pipeline_textures.in_set(RenderSet::PrepareResources),
        );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<SaikoRenderPipeline>();
    }
}

//==============================================================================
//             SaikoRenderPipeline
//==============================================================================

#[derive(Resource)]
pub struct SaikoRenderPipeline {
    pub(crate) pipeline: CachedRenderPipelineId,
    pub(crate) blit_pipeline: CachedRenderPipelineId,
    pub(crate) bind_group_layout: BindGroupLayout,
    pub(crate) blit_bind_group_layout: BindGroupLayout,
    pub(crate) render_textures: HashMap<Entity, (TextureView, u32, u32)>,
    pub(crate) fallback_image: FallbackImage,
}

impl FromWorld for SaikoRenderPipeline {
    fn from_world(world: &mut World) -> Self {
        let fallback_image = FallbackImage::from_world(world);
        let render_device = world.resource::<RenderDevice>();

        let bind_group_layout = SaikoBuffer::bind_group_layout(render_device);

        let blit_bind_group_layout = render_device.create_bind_group_layout(
            "blit_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: false }),
                    sampler(SamplerBindingType::NonFiltering),
                ),
            ),
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
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
            depth_stencil: None,
        };

        let blit_pipeline = RenderPipelineDescriptor {
            label: Some("SaikoUI Blit Pipeline".into()),
            layout: vec![blit_bind_group_layout.clone()],
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader: BLIT_SHADER_HANDLE,
                shader_defs: vec![],
                entry_point: "fs_main".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
            depth_stencil: None,
        };

        // This will add the pipeline to the cache and queue it's creation
        let pipeline = world
            .resource::<PipelineCache>()
            .queue_render_pipeline(pipeline);
        let blit_pipeline = world
            .resource::<PipelineCache>()
            .queue_render_pipeline(blit_pipeline);

        //This is where I would create bind group layouts if I had them
        SaikoRenderPipeline {
            pipeline,
            blit_pipeline,
            bind_group_layout,
            blit_bind_group_layout,
            render_textures: HashMap::new(),
            fallback_image,
        }
    }
}

//==============================================================================
//             Update Pipeline System
//==============================================================================

fn update_pipeline_textures(
    mut pipeline: ResMut<SaikoRenderPipeline>,
    render_device: ResMut<RenderDevice>,
    // render_queue: ResMut<RenderQueue>,
    view_targets: Query<(Entity, &ViewTarget)>,
) {
    for (view_target_entity, view_target) in view_targets.iter() {
        let (current_width, current_height) = pipeline
            .render_textures
            .get(&view_target_entity)
            .map(|(_, width, height)| (*width, *height))
            .unwrap_or((
                view_target.main_texture().width(),
                view_target.main_texture().height(),
            ));
        
        if current_width == view_target.main_texture().width()
            && current_height == view_target.main_texture().height()
            && pipeline.render_textures.contains_key(&view_target_entity)
        {
            continue;
        }

        println!("Updating Pipeline Render Textures");
        println!("Texture Width : {} | {}", view_target.main_texture().width(), current_width);
        println!("Texture Height : {} | {}", view_target.main_texture().height(), current_height);

        let texture = render_device.create_texture(&TextureDescriptor {
            label: Some(format!("SaikoUI Render Texture {:?}", view_target_entity).as_str()),
            size: Extent3d {
                width: view_target.main_texture().width(),
                height: view_target.main_texture().height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[TextureFormat::bevy_default()],
        });

        let texture_view = texture.create_view(&TextureViewDescriptor {
            label: Some(format!("SaikoUI Render Texture {:?}", view_target_entity).as_str()),
            ..Default::default()
        });

        pipeline
            .render_textures
            .insert(view_target_entity, (texture_view, view_target.main_texture().width(), view_target.main_texture().height()));
        
        println!("Render Textures: {}", pipeline.render_textures.len());
    }
}
