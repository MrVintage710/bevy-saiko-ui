use bevy::prelude::*;
use bevy::render::render_graph::{RenderLabel, RenderSubGraph};
use bevy::render::{
    render_graph::ViewNode,
    render_resource::{PipelineCache, RenderPassDescriptor},
    view::ViewTarget,
};

use crate::render::pipeline::SaikoRenderPipeline;

use super::buffer::SaikoPreparedBuffer;
use super::SaikoRenderTarget;

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
    type ViewQuery = (
        &'static ViewTarget,
        &'static SaikoRenderTarget,
        &'static SaikoPreparedBuffer
    );

    fn run<'w>(
        &self,
        graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        view_query: bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let (view_target, saiko_render_target, prepared_buffer) = view_query;

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

        //If the shaping data has been loaded into the bind group, render it
        // if let Some(prepared_bind_group) = &saiko_pipeline_resource.bind_groups {
        let bind_group = &prepared_buffer.0.bind_group;

        //Create the render pass. This is what will render the final result.
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: "SaikoUI Render Pass".into(),
            color_attachments: &[Some(view_target.get_color_attachment())],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        //Set the pipeline to be rendered and attach the bind group
        render_pass.set_render_pipeline(saiko_pipeline);
        render_pass.set_bind_group(0, bind_group, &[]);

        //Send it baby!
        render_pass.draw(0..3, 0..1);
        // }

        Ok(())
    }
}
