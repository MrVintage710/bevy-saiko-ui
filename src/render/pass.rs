use bevy::render::{render_graph::{RenderLabel, RenderSubGraph, ViewNode}, render_resource::{BufferInitDescriptor, BufferUsages, PipelineCache, RenderPassDescriptor}, view::ViewTarget};
use bevy::prelude::*;

use crate::render::{pipeline::SaikoRenderPipeline, vertex::VertexRect};

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
    );

    fn run<'w>(
        &self,
        graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        view_query: bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        println!("Rendering");
        
        let (view_target) = view_query;
        
        //Get Pipeline from Resources
        let saiko_pipeline_resource = world.resource::<SaikoRenderPipeline>();
        
        //Get the pipeline cache
        let pipeline_cache = world.resource::<PipelineCache>();
        
        //Get the pipeline from 
        let Some(saiko_pipeline) = pipeline_cache.get_render_pipeline(saiko_pipeline_resource.pipeline) 
            else { return Ok(()) };
        
        //Create a bind group with all of the drawables.
        // let bind_group = render_context.render_device().create_bind_group(
        //     "SaikoUI BindGroup", 
        //     &saiko_pipeline_resource.bind_group_layout, 
        //     entries
        // );
            
        let test_rect = VertexRect {
            position: [0.0, 0.0, 0.0],
            size: [100.0, 100.0],
            color: [1.0, 0.0, 0.0, 1.0],
        };
        
        let vertex_buffer = render_context
            .render_device()
            .create_buffer_with_data(&BufferInitDescriptor {
                label: Some("SaikoUI Vertex Buffer"),
                contents: bytemuck::cast_slice(&[test_rect]),
                usage: BufferUsages::VERTEX,
            });
        
        //Create the render pass. This is what will render the final result.
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: "SaikoUI Render Pass".into(),
            color_attachments: &[
                Some(view_target.0.get_color_attachment())
            ],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        
        render_pass.set_render_pipeline(saiko_pipeline);
        // render_pass.set_bind_group()
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..1, 0..1);
        
        Ok(())
    }
}