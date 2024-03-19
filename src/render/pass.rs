use bevy::prelude::*;
use bevy::render::{
    render_graph::{RenderLabel, RenderSubGraph, ViewNode},
    render_resource::{
        AsBindGroup, BindGroupEntries, BindGroupEntry, BindingResource, BufferDescriptor,
        BufferInitDescriptor, BufferUsages, Operations, PipelineCache, RenderPassColorAttachment,
        RenderPassDescriptor,
    },
    view::ViewTarget,
};

use crate::render::pipeline::SaikoRenderPipeline;

use super::buffer::SaikoBuffer;

//==============================================================================
//             SaikoRenderNode
//==============================================================================

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
pub struct SaikoSubGraph;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct SaikoRenderLabel;

#[derive(Default)]
pub struct SaikoRenderNode;

impl ViewNode for SaikoRenderNode {
    type ViewQuery = (&'static ViewTarget);

    fn run<'w>(
        &self,
        graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        view_query: bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let (view_target) = view_query;

        //Get Pipeline from Resources
        let saiko_pipeline_resource = world.resource::<SaikoRenderPipeline>();

        //Get the pipeline cache
        let pipeline_cache = world.resource::<PipelineCache>();

        //Get the pipeline from the pipeline cache
        let Some(saiko_pipeline) =
            pipeline_cache.get_render_pipeline(saiko_pipeline_resource.pipeline)
        else {
            return Ok(());
        };

        // let test_rect = RectBuffer {
        //     position: Vec3::ZERO,
        //     size: Vec2::new(100.0, 100.0),
        //     color: Vec4::new(1.0, 1.0, 0.0, 0.5),
        //     corners: Vec4::ZERO,
        // };

        // let saiko_buffer = SaikoBuffer {
        //     rectangles: vec![test_rect],
        // };

        // let buffer = render_context.render_device().create_buffer_with_data(&BufferInitDescriptor {
        //     label: Some("Test Rect Buffer"),
        //     usage: BufferUsages::STORAGE,
        //     contents: test_rect.to_buffer(),
        // });

        if let Some(prepared_bind_group) = &saiko_pipeline_resource.prepared_bind_group {
            println!("Rendering");
            let bind_group = &prepared_bind_group.bind_group;

            //Create the render pass. This is what will render the final result.
            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: "SaikoUI Render Pass".into(),
                color_attachments: &[Some(view_target.get_color_attachment())],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_render_pipeline(saiko_pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}
