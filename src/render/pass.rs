use bevy::core_pipeline::blit::BlitPipeline;
use bevy::prelude::*;
use bevy::render::render_graph::{RenderLabel, RenderSubGraph};
use bevy::render::render_resource::{BindGroupEntries, Operations, RenderPassColorAttachment};
use bevy::render::{
    render_graph::ViewNode,
    render_resource::{PipelineCache, RenderPassDescriptor},
    view::ViewTarget,
};

use crate::render::pipeline::SaikoRenderPipeline;

use super::buffer::SaikoPreparedBuffer;

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
        Entity,
        &'static ViewTarget,
        Option<&'static SaikoPreparedBuffer>,
    );

    fn run<'w>(
        &self,
        graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        view_query: bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let (entity, view_target, prepared_buffer) = view_query;

        //Get Pipelines from Resources
        let saiko_pipeline_resource = world.resource::<SaikoRenderPipeline>();

        //Get the pipeline cache
        let pipeline_cache = world.resource::<PipelineCache>();

        //Get the pipeline from the pipeline cache
        let Some(saiko_pipeline) =
            pipeline_cache.get_render_pipeline(saiko_pipeline_resource.pipeline)
        else {
            return Ok(());
        };

        let Some(blit_pipeline) =
            pipeline_cache.get_render_pipeline(saiko_pipeline_resource.blit_pipeline)
        else {
            return Ok(());
        };

        let (render_texture, _, _) = saiko_pipeline_resource
            .render_textures
            .get(&entity)
            .unwrap();

        //If the shaping data has been loaded into the bind group, render it
        // if let Some(prepared_bind_group) = &saiko_pipeline_resource.bind_groups {
        if let Some(prepared_buffer) = prepared_buffer {
            println!("Rendering to texture!");
            //Get the bind group from the prepared buffer
            let bind_group = &prepared_buffer.0.bind_group;

            //Create the render pass. This is what will render the final result.
            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: "SaikoUI Render Pass".into(),
                // color_attachments: &[Some(view_target.get_color_attachment())],
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: render_texture,
                    resolve_target: None,
                    ops: Operations::default(),
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            //Set the pipeline to be rendered and attach the bind group
            render_pass.set_render_pipeline(saiko_pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);

            //Send it baby!
            render_pass.draw(0..3, 0..1);
        }

        let blit_pipeline_resource = world.resource::<BlitPipeline>();

        let blit_bind_group = render_context.render_device().create_bind_group(
            None,
            &saiko_pipeline_resource.blit_bind_group_layout,
            &BindGroupEntries::sequential((render_texture, &blit_pipeline_resource.sampler)),
        );

        let mut blit_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: "SaikoUI Blit Render Pass".into(),
            color_attachments: &[Some(view_target.get_unsampled_color_attachment())],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        blit_pass.set_render_pipeline(blit_pipeline);
        blit_pass.set_bind_group(0, &blit_bind_group, &[]);

        blit_pass.draw(0..3, 0..1);

        // let bind_group = &prepared_buffer.0.bind_group;

        // //Create the render pass. This is what will render the final result.
        // let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
        //     label: "SaikoUI Render Pass".into(),
        //     // color_attachments: &[Some(view_target.get_color_attachment())],
        //     color_attachments: &[
        //         Some(RenderPassColorAttachment {
        //             view: render_texture,
        //             resolve_target: None,
        //             ops: Operations::default()
        //         })
        //     ],
        //     depth_stencil_attachment: None,
        //     timestamp_writes: None,
        //     occlusion_query_set: None,
        // });

        // //Set the pipeline to be rendered and attach the bind group
        // render_pass.set_render_pipeline(saiko_pipeline);
        // render_pass.set_bind_group(0, bind_group, &[]);

        // //Send it baby!
        // render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
