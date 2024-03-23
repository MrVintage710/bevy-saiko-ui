//==============================================================================
//  SaikoComponent is how you define a component that can be drawn by the UI
//  system. The component will keep its own state, but will call the render
//  funtion whenever a render is called.
//==============================================================================

mod rect;

use std::marker::PhantomData;

use bevy::{prelude::*, render::{Extract, RenderApp}};

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
    mut render_targets : Query<&mut SaikoRenderTarget>,
    query : Extract<Query<&T>>
) {
    for mut render_target in render_targets.iter_mut() {
        for component in query.iter() {
            component.render(&mut render_target.1);
        }
    }
}