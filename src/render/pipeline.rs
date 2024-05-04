use std::num::NonZeroU32;

use bevy::{
    core_pipeline::
        fullscreen_vertex_shader::fullscreen_shader_vertex_state
    ,
    prelude::*,
    render::{
        render_resource::{
            binding_types::{sampler, texture_2d}, AddressMode, AsBindGroup, BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntry, BindingType, BlendState, CachedRenderPipelineId, ColorTargetState, ColorWrites, Extent3d, FilterMode, FragmentState, MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, ShaderType, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension
        },
        renderer::RenderDevice,
        texture::{BevyDefault, FallbackImage},
        view::ViewTarget,
        Render, RenderApp, RenderSet,
    },
    utils::HashMap,
};

use super::{buffer::{ManualShaderType, SaikoBuffer}, font::sdf::GpuSaikoFonts, BLIT_SHADER_HANDLE, SAIKO_SHADER_HANDLE};

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
    pub(crate) font_bind_group_layout: BindGroupLayout,
    pub(crate) font_sampler: Sampler,
    pub(crate) render_textures: HashMap<Entity, (TextureView, u32, u32)>,
    pub(crate) fallback_image: FallbackImage,
}

impl FromWorld for SaikoRenderPipeline {
    fn from_world(world: &mut World) -> Self {
        let fallback_image = FallbackImage::from_world(world);
        let render_device = world.resource::<RenderDevice>();
        
        
        //This is some weird hacky code to get bind group layouts to work
        let bind_group_layout = SaikoBuffer::bind_group_layout(render_device);
        let font_bind_group_layout = render_device.create_bind_group_layout(
            "SaikoFontAtlasSdf",
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture { 
                        sample_type: bevy::render::render_resource::TextureSampleType::Float { filterable: true }, 
                        view_dimension: TextureViewDimension::D2Array, 
                        multisampled: false 
                    },
                    count: Some(NonZeroU32::new(GpuSaikoFonts::MAX_FONTS).unwrap()),
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ]
        );
        
        let font_sampler = render_device.create_sampler(&SamplerDescriptor {
            label: Some("Saiko SDF Font Atlas Sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            // mipmap_filter: todo!(),
            // lod_min_clamp: todo!(),
            // lod_max_clamp: todo!(),
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
            ..Default::default()
        });
        
        // let font_atlas_sampler = render_device.create_sampler(&SamplerDescriptor {
        //     label: Some("Saiko SDF Font Atlas Sampler"),
        //     address_mode_u: AddressMode::ClampToEdge,
        //     address_mode_v: AddressMode::ClampToEdge,
        //     address_mode_w: AddressMode::ClampToEdge,
        //     // mag_filter: FilterMode::Linear,
        //     // min_filter: FilterMode::Linear,
        //     // mipmap_filter: todo!(),
        //     // lod_min_clamp: todo!(),
        //     // lod_max_clamp: todo!(),
        //     compare: None,
        //     anisotropy_clamp: 1,
        //     border_color: None,
        //     ..Default::default()
        // });
        
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
            layout: vec![bind_group_layout.clone(), font_bind_group_layout.clone()],
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
            font_bind_group_layout,
            font_sampler,
            render_textures: HashMap::new(),
            fallback_image
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
    }
}
