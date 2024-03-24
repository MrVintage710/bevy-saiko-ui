//==============================================================================
//  SaikoComponent is how you define a component that can be drawn by the UI
//  system. The component will keep its own state, but will call the render
//  funtion whenever a render is called.
//==============================================================================

mod rect;

use std::marker::PhantomData;

use bevy::{prelude::*, render::{view::RenderLayers, Extract, RenderApp}};

use crate::render::{buffer::SaikoBuffer, SaikoRenderTarget};

//==============================================================================
//          SaikoComponent
//==============================================================================

pub trait SaikoComponent : Component {
    fn render(&self, buffer : &mut SaikoBuffer);
}

//==============================================================================
//          SaikoComponentPlugin
//==============================================================================

#[derive(Default)]
pub struct SaikoComponentPlugin<T : SaikoComponent>(PhantomData<T>);

impl <T : SaikoComponent> Plugin for SaikoComponentPlugin<T> {
    fn build(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        
        render_app.add_systems(
            ExtractSchedule,
            extract_components::<T>.after(crate::render::extract_cameras_for_render),
        );
    }
}

//==============================================================================
//          SaikoComponent Systems
//==============================================================================

fn extract_components<T : SaikoComponent>(
    mut render_targets : Query<(&mut SaikoRenderTarget, Option<&RenderLayers>)>,
    query : Extract<Query<(&T, Option<&RenderLayers>, Option<&InheritedVisibility>)>>
) {
    for (mut render_target, render_target_layers) in render_targets.iter_mut() {
        for (component, component_render_layers, component_visability) in query.iter() {
            let visable = component_visability.map_or(true, |v| v.get());
            let on_layer = match (render_target_layers, component_render_layers) {
                (Some(render_layers), Some(component_render_layers)) => render_layers.intersects(component_render_layers),
                (None, Some(_)) |
                (Some(_), None) => false,
                _ => true
            };
            
            if on_layer && visable {
                component.render(&mut render_target.1);
            }
        }
    }
}