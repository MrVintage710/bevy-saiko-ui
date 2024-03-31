pub mod buffer;
mod pass;
mod pipeline;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        core_3d::graph::{Core3d, Node3d},
    },
    prelude::*,
    render::{
        render_asset::RenderAssets, render_graph::{RenderGraph, RunGraphOnViewNode, ViewNodeRunner}, render_resource::AsBindGroup, renderer::RenderDevice, view::RenderLayers, Extract, Render, RenderApp, RenderSet
    },
};

use crate::{common::MarkSaikoUiDirty, render::{
    pass::{SaikoRenderLabel, SaikoSubGraph},
    pipeline::{SaikoRenderPipeline, SaikoRenderPipelinePlugin},
}};

use self::{buffer::{SaikoBuffer, SaikoPreparedBuffer}, pass::SaikoRenderNode};

//==============================================================================
//  This is the render module for Saiko UI. It has been inspired by the
//  renderer that has been made for the Zed editor called GPUI. Here is
//  a link to a blog post about their rendering: https://zed.dev/blog/videogame
//
//  TODO: Rewrite the following line
//
//  The following are the RenderingNode and the RenderPipeline.
//  The RenderingNode is a struct that defines how the ui is renderded in
//  the render world. The RenderPipeline is chached data that is used to
//  render, and is saved between render ticks.
//==============================================================================

pub const SAIKO_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(11079037277321826659);
pub const BLIT_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(11079030077321896629);

//==============================================================================
//             SaikoRenderPlugin
//==============================================================================

pub struct SaikoRenderPlugin;

impl Plugin for SaikoRenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SAIKO_SHADER_HANDLE, "saiko.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, BLIT_SHADER_HANDLE, "blit.wgsl", Shader::from_wgsl);

        app.init_resource::<SaikoRenderState>();
        
        app
            .add_plugins(SaikoRenderPipelinePlugin)
            
            .add_systems(First, reset_saiko_render_state)
            .add_systems(Last, update_saiko_render_state)
        ;

        // We need to get the render app from the main app
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(ExtractSchedule, (extract_cameras_for_render, apply_deferred));
        render_app.add_systems(Render, prepare_prepare_bind_groups.in_set(RenderSet::PrepareResources));

        let ui_graph_2d = get_ui_graph(render_app);
        let ui_graph_3d = get_ui_graph(render_app);
        let mut graph = render_app.world.resource_mut::<RenderGraph>();

        if let Some(graph_2d) = graph.get_sub_graph_mut(Core2d) {
            graph_2d.add_sub_graph(SaikoSubGraph, ui_graph_2d);
            graph_2d.add_node(SaikoRenderLabel, RunGraphOnViewNode::new(SaikoSubGraph));
            graph_2d.add_node_edge(Node2d::MainPass, SaikoRenderLabel);
            graph_2d.add_node_edge(Node2d::EndMainPassPostProcessing, SaikoRenderLabel);
            graph_2d.add_node_edge(SaikoRenderLabel, Node2d::Upscaling);
        }

        if let Some(graph_3d) = graph.get_sub_graph_mut(Core3d) {
            graph_3d.add_sub_graph(SaikoSubGraph, ui_graph_3d);
            graph_3d.add_node(SaikoRenderLabel, RunGraphOnViewNode::new(SaikoSubGraph));
            graph_3d.add_node_edge(Node3d::EndMainPass, SaikoRenderLabel);
            graph_3d.add_node_edge(Node3d::EndMainPassPostProcessing, SaikoRenderLabel);
            graph_3d.add_node_edge(SaikoRenderLabel, Node3d::Upscaling);
        }
    }
}

fn get_ui_graph(render_app: &mut App) -> RenderGraph {
    let mut saiko_graph = RenderGraph::default();
    let view_node = ViewNodeRunner::new(SaikoRenderNode, &mut render_app.world);
    saiko_graph.add_node(SaikoRenderLabel, view_node);
    saiko_graph
}

//==============================================================================
//             SaikoRenderIsDirty
//==============================================================================

#[derive(Resource)]
pub struct SaikoRenderState{
    is_dirty: bool,
}

impl Default for SaikoRenderState {
    fn default() -> Self {
        Self {
            is_dirty: true,
        }
    }
}

impl SaikoRenderState {
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
}

fn update_saiko_render_state(
    mut state : ResMut<SaikoRenderState>,
    dirty : EventReader<MarkSaikoUiDirty>
) {
    if !dirty.is_empty() {
        state.is_dirty = true;
    }
}

fn reset_saiko_render_state(
    mut state : ResMut<SaikoRenderState>
) {
    state.is_dirty = false;
}

//==============================================================================
//             SaikoRenderTarget
//==============================================================================

#[derive(Component)]
pub struct SaikoRenderTarget(pub Option<RenderLayers>, pub SaikoBuffer);

//==============================================================================
//             SaikoUi Render Systems
//==============================================================================

fn extract_cameras_for_render(
    mut commands : Commands,
    mut has_initialized : Local<bool>,
    cameras : Extract<
        Query<(Entity, Option<&RenderLayers>), With<Camera>>
    >,
    ui_dirty : Extract<
        Res<SaikoRenderState>
    >,
) {
    if ui_dirty.is_dirty() || !*has_initialized {
        for (entity, render_layers) in cameras.iter() {
            let mut cam_entity = commands.get_or_spawn(entity);
            let render_layers = render_layers.map(|value| value.clone());
            cam_entity.insert(SaikoRenderTarget(render_layers, SaikoBuffer::default()));
        }
        
        *has_initialized = true;
    }
}

fn prepare_prepare_bind_groups(
    mut commands : Commands,
    saiko_pipeline: ResMut<SaikoRenderPipeline>,
    render_targets : Query<(Entity, &SaikoRenderTarget)>,
    images: Res<RenderAssets<Image>>,
    render_device: Res<RenderDevice>,
) {
    for (render_target_entity, render_target) in render_targets.iter() {
        println!("Preparing render texture");
        let Ok(prepared_bind_group) = render_target.1.as_bind_group(
            &saiko_pipeline.bind_group_layout,
            render_device.as_ref(),
            images.as_ref(),
            &saiko_pipeline.fallback_image,
        ) else { continue };
        
        commands.entity(render_target_entity).insert(SaikoPreparedBuffer(prepared_bind_group));
    }
}
