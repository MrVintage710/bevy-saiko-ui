

use bevy::{ecs::{query::{QueryData, QueryFilter}, system::QueryLens}, prelude::*};

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

//==============================================================================
//          get_hierarchy function
//==============================================================================

///This function returns a vec of all the entities in the hierarchy with the given entity as the root.
///The entities are sorted in a depth first fasion with the root entity being the first element.
///The query that you pass in must beable to be transumted into a lens of type `Query<Option<&Children>>` or else it will panic.
pub fn get_all_children<Q : QueryData, F : QueryFilter>(entity : Entity, query : &mut Query<'_, '_, Q, F>) -> Vec<Entity> {
    let mut lens = query.transmute_lens::<Option<&Children>>();
    get_all_children_r(entity, &lens.query())
}

///This function returns all children in a vec, sorted in a depth fisrt fasion.
fn get_all_children_r (
    current_entity : Entity,
    entities : &Query<Option<&Children>>
) -> Vec<Entity> {
    let mut ancestors = vec![current_entity];
    let Ok(children) = entities.get(current_entity) else { panic!("Entity not part of the query!")};
    let Some(children) = children else { return ancestors };
    for entity in children.iter() {
        ancestors.append(&mut get_all_children_r(*entity, entities));
    }
    ancestors
}