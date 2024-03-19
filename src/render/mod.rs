mod buffer;
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
        render_asset::RenderAssets,
        render_graph::{RenderGraph, RunGraphOnViewNode, ViewNodeRunner},
        render_resource::AsBindGroup,
        renderer::RenderDevice,
        Render, RenderApp,
    },
};

use crate::render::{
    buffer::RectBuffer,
    pass::{SaikoRenderLabel, SaikoSubGraph},
    pipeline::SaikoRenderPipeline,
};

use self::{buffer::SaikoBuffer, pass::SaikoRenderNode};

//==============================================================================
//  This is the render module for Saiko UI. It has been inspired by the
//  renderer that has been made for the Zed editor called GPUI. Here is
//  a link to a blog post about their rendering: https://zed.dev/blog/videogame
//
//  The following are the RenderingNode and the RenderPipeline.
//  The RenderingNode is a struct that defines how the ui is renderded in
//  the render world. The RenderPipeline is chached data that is used to
//  render, and is saved between render ticks.
//==============================================================================

pub const SAIKO_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(11079037277321826659);

//==============================================================================
//             SaikoRenderPlugin
//==============================================================================

pub struct SaikoRenderPlugin;

impl Plugin for SaikoRenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SAIKO_SHADER_HANDLE, "saiko.wgsl", Shader::from_wgsl);

        // app

        // ;

        // We need to get the render app from the main app
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(Render, prepare_ui_render_texture);

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

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<SaikoRenderPipeline>();
    }
}

fn get_ui_graph(render_app: &mut App) -> RenderGraph {
    let mut saiko_graph = RenderGraph::default();
    let view_node = ViewNodeRunner::new(SaikoRenderNode, &mut render_app.world);
    saiko_graph.add_node(SaikoRenderLabel, view_node);
    saiko_graph
}

//==============================================================================
//             SaikoUi Render Systems
//==============================================================================

fn extract_cameras_for_render() {}

fn prepare_ui_render_texture(
    mut saiko_pipeline: ResMut<SaikoRenderPipeline>,
    images: Res<RenderAssets<Image>>,
    render_device: Res<RenderDevice>,
) {
    if saiko_pipeline.prepared_bind_group.is_none() {
        println!("Creating a thing");

        let buffer = SaikoBuffer {
            rectangles: vec![RectBuffer::default()
                .with_size((100.0, 100.0))
                .with_color((1.0, 0.0, 0.0, 0.5))],
        };

        let Ok(bind_group) = buffer.as_bind_group(
            &saiko_pipeline.bind_group_layout,
            render_device.as_ref(),
            images.as_ref(),
            &saiko_pipeline.fallback_image,
        ) else {
            return;
        };
        saiko_pipeline.prepared_bind_group = Some(bind_group);
    }
}
