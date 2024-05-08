//==============================================================================
//  SaikoComponent is how you define a component that can be drawn by the UI
//  system. The component will keep its own state, but will call the render
//  funtion whenever a render is called.
//==============================================================================

pub mod rect;
pub mod cache;

use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::{view::RenderLayers, Extract, RenderApp},
};

use crate::render::{font::sdf::SaikoFontSdf, SaikoRenderState, SaikoRenderTarget};

use super::{context::SaikoRenderContext, node::SaikoNode};

//==============================================================================
//          SaikoComponent
//==============================================================================

pub trait SaikoComponent: Component {
    type Data;
    
    fn update_cache(&self, cache : &mut dyn SaikoComponentCache<Data = Self::Data>);
    
    fn render(&self, buffer: &mut SaikoRenderContext<'_>);
    
    fn should_auto_update() -> bool { true }
}

//==============================================================================
//          SaikoComponentCache
//==============================================================================

pub trait SaikoComponentCache : Resource {
    type Data;
    
    fn push(&mut self, data : Self::Data);
    
    fn get(&self) -> &Self::Data;
}

//==============================================================================
//          SaikoComponentPlugin
//==============================================================================

pub struct SaikoComponentPlugin<T: SaikoComponent>(PhantomData<T>);

impl<T: SaikoComponent> Plugin for SaikoComponentPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Last, component_change_detection::<T>)
        ;
        
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(
            ExtractSchedule,
            extract_components::<T>.after(apply_deferred),
        );
    }
}

impl<T: SaikoComponent> Default for SaikoComponentPlugin<T> {
    fn default() -> Self {
        SaikoComponentPlugin(PhantomData)
    }
}

//==============================================================================
//          SaikoComponent Systems
//==============================================================================

fn extract_components<T: SaikoComponent>(
    mut render_targets: Query<(&mut SaikoRenderTarget, Option<&RenderLayers>)>,
    query : Extract<Query<(&T, &SaikoNode, Option<&RenderLayers>, Option<&InheritedVisibility>)>>,
    fonts : Extract<Res<Assets<SaikoFontSdf>>>
) {
    for (mut render_target, render_target_layers) in render_targets.iter_mut() {
        for (component, node, component_render_layers, component_visability) in query.iter() {
            let visable = component_visability.map_or(true, |v| v.get());
            let on_layer = match (render_target_layers, component_render_layers) {
                (Some(render_layers), Some(component_render_layers)) => {
                    render_layers.intersects(component_render_layers)
                }
                (None, Some(_)) | (Some(_), None) => false,
                _ => true,
            };

            if on_layer && visable {
                //In future releases, this code should happen somewhere else
                let mut render_context = SaikoRenderContext::new(&mut render_target.1, fonts.as_ref(), *node.bounds());
                component.render(&mut render_context);
            }
        }
    }
}

fn component_change_detection<T: SaikoComponent>(
    mut render_state : ResMut<SaikoRenderState>,
    components : Query<(Ref<T>, Ref<SaikoNode>)>
) {
    for (component, node) in components.iter() {
        if T::should_auto_update() && (node.is_changed() || component.is_changed()) {
            render_state.mark_dirty();
        }
    }
}
