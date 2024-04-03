
use bevy::{ecs::system::QueryLens, prelude::*};

//==============================================================================
//          Find genderation
//==============================================================================

pub fn find_generation(entity : Entity, entities : &mut QueryLens<Option<&Parent>>) -> u32 {
    find_generation_r(0, entity, &entities.query())
}

fn find_generation_r (
    current_generation : u32,
    current_entity : Entity,
    entities : &Query<Option<&Parent>>
) -> u32 {
    let Ok(parent) = entities.get(current_entity) else { return current_generation };
    let Some(parent) = parent else { return current_generation};
    find_generation_r(current_generation + 1, **parent, entities)
}