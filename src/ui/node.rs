//==============================================================================
//  SaikoNode is a needed component for all UI to have. This is the component
//  that dertmines the ui's location on the screen and whether the ui needs to
//  be updated or not.
//==============================================================================

use bevy::{ecs::reflect, prelude::*, utils::HashSet, window::PrimaryWindow};

use crate::common::{bounds::Bounds, util::get_all_children};

use super::position::RelativePosition;

//==============================================================================
//          SaikoNodePlugin
//==============================================================================

pub(crate) struct SaikoNodePlugin;

impl Plugin for SaikoNodePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, update_node_bounds)
            
            .register_type::<SaikoNode>()
        
        ;
        
    }
}

//==============================================================================
//          SaikoNode Component
//==============================================================================

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SaikoNode {
    #[reflect(ignore)]
    bounds: Bounds,
    #[reflect(default)]
    position: RelativePosition,
    is_dirty: bool,
}

impl SaikoNode {
    pub fn new(position: RelativePosition) -> Self {
        SaikoNode {
            bounds: Bounds::default(),
            position,
            is_dirty: true,
        }
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }
    
    pub fn calc_bounds(&mut self, parent: &Bounds) {
        self.position.calc_bounds(parent, &mut self.bounds);
    }
}

//==============================================================================
//          SaikoNode Systems
//==============================================================================

fn update_node_bounds(
    mut nodes: Query<(Entity, &mut SaikoNode, Option<&Children>, Option<&Parent>), Changed<SaikoNode>>,
    primary_window : Query<&Window, With<PrimaryWindow>>
) {
    if nodes.iter().next().is_none() { return }
    println!("Updating Node Bounds");
    
    let Ok(window) = primary_window.get_single() else { return };
    // let window_bounds = Bounds::new(Vec2::ZERO, Vec2::new(window.width(), window.height()), 0);
    
    let entities_to_update = nodes
        .iter()
        .map(|query| query.0)
        .collect::<Vec<_>>();
    
    
    let families_to_update = entities_to_update
        .iter()
        .map(|entity| get_all_children(*entity, &mut nodes))
        .collect::<Vec<_>>();
    
    let mut updated_nodes = HashSet::new();
    
    for family in families_to_update {
        for entity in family {
            if !updated_nodes.insert(entity) || !nodes.contains(entity) { continue }
            
            let mut parent_bounds = Bounds::new(Vec2::ZERO, Vec2::new(window.width(), window.height()), 0);
            
            let parent = nodes.get(entity).map(|t| t.3.map(|parent| **parent)).unwrap_or(None);
            if let Some(parent) = parent {
                if let Ok((_, parent_node, _, _)) = nodes.get_mut(parent) {
                    parent_bounds = parent_node.bounds;
                }
            }
            
            let Ok((_, mut node, _, _)) = nodes.get_mut(entity) else { continue };
            node.calc_bounds(&parent_bounds);
        }
    }    
}

// fn set_bounds(entity : Entity, query : &mut Query<(Entity, &mut SaikoNode, Option<&Parent>)>, bounds : Bounds, updated_nodes : &mut HashSet<Entity>) {
//     if updated_nodes.contains(&entity) { return }
//     let Ok((_, mut node, _)) = query.get_mut(entity) else { return };
//     node.bounds = bounds;
//     updated_nodes.insert(entity);
// }

// fn calc_bounds(
//     entity_pair : (Entity, Option<Entity>),
//     nodes: &mut Query<(Entity, &mut SaikoNode, Option<&Parent>)>,
//     default_bounds : Bounds
// ) -> Bounds {
//     if let Some(parent) = entity_pair.1 {
        
//     }
// }
