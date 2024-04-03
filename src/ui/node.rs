//==============================================================================
//  SaikoNode is a needed component for all UI to have. This is the component
//  that dertmines the ui's location on the screen and whether the ui needs to
//  be updated or not.
//==============================================================================

use bevy::{ecs::reflect, prelude::*, window::PrimaryWindow};

use crate::common::bounds::Bounds;

use super::position::RelativePosition;

//==============================================================================
//          SaikoNodePlugin
//==============================================================================

pub(crate) struct SaikoNodePlugin;

impl Plugin for SaikoNodePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SaikoNode>();
    }
}

//==============================================================================
//          SaikoNode Component
//==============================================================================

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SaikoNode {
    // #[reflect(ignore)]
    bounds: Bounds,
    #[reflect(default)]
    position: RelativePosition,
}

impl SaikoNode {
    pub fn new(position: RelativePosition) -> Self {
        SaikoNode {
            bounds: Bounds::default(),
            position,
        }
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

//==============================================================================
//          SaikoNode Systems
//==============================================================================

fn update_node_bounds(
    mut query: Query<(&mut SaikoNode, Option<&Parent>)>,
    primary_window : Query<&Window, With<PrimaryWindow>>
) {
    
}

// fn get_parent_bounds(
//     current_entity : Entity,
//     mut query: &Query<(&SaikoNode, &Parent)>,
//     primary_window : &Query<&Window, With<PrimaryWindow>>
// ) -> Bounds {
    
// }
