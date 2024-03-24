//==============================================================================
//  SaikoNode is a needed component for all UI to have. This is the component
//  that dertmines the ui's location on the screen and whether the ui needs to
//  be updated or not.
//==============================================================================

use bevy::prelude::*;

use crate::common::bounds::Bounds;

use super::position::RelativePosition;

//==============================================================================
//          SaikoNode Component
//==============================================================================

#[derive(Component, Reflect)]
pub struct SaikoNode {
    bounds : Bounds,
    position : RelativePosition
}

impl SaikoNode {
    pub fn new(position : RelativePosition) -> Self {
        SaikoNode {
            bounds : Bounds::default(),
            position
        }
    }
}

//==============================================================================
//          SaikoNode Systems
//==============================================================================

