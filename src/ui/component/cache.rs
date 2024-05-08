//==============================================================================
//  This module hold the logic for the component caching system. While rendering
// components, there need to be a spot to store computationally expensive data,
// like glyph shaping data. This also acts as a great spot for coping data to 
// the GPU when rendering is needed.
//==============================================================================

use bevy::{ecs::{entity::Entity, system::Resource}, utils::HashMap};

use crate::render::buffer::SaikoBuffer;

#[derive(Resource)]
pub struct ComponentCache {
    cache : HashMap<Entity, HashMap<u16, Box<dyn ComponentCacheItem + Send + Sync + 'static>>>,
    is_dirty : bool,
}

pub trait ComponentCacheItem {
    fn modify_buffer(&self, buffer: &mut SaikoBuffer);
}

pub struct TextCacheItem {
    
}