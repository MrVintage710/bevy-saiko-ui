//==============================================================================
//  SaikoComponent is how you define a component that can be drawn by the UI
//  system. The component will keep its own state, but will call the render
//  funtion whenever a render is called.
//==============================================================================

pub mod rect;

use std::{marker::PhantomData};

use bevy::{
    prelude::*,
    render::{view::RenderLayers, Extract, RenderApp}, utils::HashMap,
};

use crate::render::{buffer::SaikoBuffer, font::sdf::SaikoFontSdf, SaikoRenderState, SaikoRenderTarget};

use super::{context::SaikoRenderContext, node::SaikoNode};

pub(crate) struct SaikoComponentsPlugin;

impl Plugin for SaikoComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ComponentCache>()
        
            .add_systems(First, reset_component_cache)
        ;
        
        
    }
}

//==============================================================================
//          SaikoComponent
//==============================================================================

pub trait SaikoComponent: Component {    
    fn render(&self, buffer: &mut SaikoRenderContext<'_>);
    
    fn should_auto_update() -> bool { true }
}

//==============================================================================
//          SaikoComponentCache
//==============================================================================

#[derive(Resource, Default)]
pub struct ComponentCache {
    cache : HashMap<Entity, ComponentCacheItem>,
    is_dirty : bool,
}

impl ComponentCache {
    pub fn set_item(&mut self, entity : &Entity, buffer : SaikoBuffer, render_layers : Option<RenderLayers>) {
        let item = ComponentCacheItem {
            buffer,
            render_layers,
        };
        self.cache.insert(*entity, item);
        self.is_dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    
    pub fn get_cache(&self) -> &HashMap<Entity, ComponentCacheItem> {
        &self.cache
    }
}

#[derive(Default)]
pub struct ComponentCacheItem {
    pub buffer : SaikoBuffer,
    pub render_layers : Option<RenderLayers>,
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

fn reset_component_cache(
    mut cache : ResMut<ComponentCache>
) {
    cache.is_dirty = false;
}

fn component_change_detection<T: SaikoComponent>(
    mut render_state : ResMut<SaikoRenderState>,
    mut component_cache : ResMut<ComponentCache>,
    fonts : Res<Assets<SaikoFontSdf>>,
    components : Query<(Entity, Ref<T>, Ref<SaikoNode>, Option<&RenderLayers>)>,
) {
    for (entity, component, node, render_layers) in components.iter() {
        if T::should_auto_update() && (node.is_changed() || component.is_changed()) {
            println!("Updating Component");
            let mut buffer = SaikoBuffer::default();
            let mut render_context = SaikoRenderContext::new(&mut buffer, fonts.as_ref(), *node.bounds());
            component.render(&mut render_context);
            
            drop(render_context);
            
            component_cache.set_item(&entity, buffer, render_layers.map(|layers| layers.clone()));
            
            render_state.mark_dirty();
        }
    }
}
