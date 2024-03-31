//==============================================================================
//  This is the Bounds Plugin. SaikoBounds is a struct that is the core for
//  supplying the renderer where to render certain UI. The shaping and sdf's
//  are derived from a ui's bounds.
//==============================================================================

use bevy::prelude::*;

//==============================================================================
//          SaikoBounds struct
//==============================================================================

/// SaikoBounds describes an Rectagul that is axis aligned that determines
/// the bounds of a UI element.
#[derive(Reflect, Default)]
pub struct Bounds {
    /// The center of the bounds.
    pub center: Vec2,
    /// The size of the bounds. This is the width and height from edge to edge.
    pub size: Vec2,
    /// The z_index of the bounds. This is used to determine the rendering order.
    pub z_index: i32,
}

impl Bounds {
    pub fn new(center: Vec2, size: Vec2, z_index: i32) -> Self {
        Bounds {
            center,
            size,
            z_index,
        }
    }
}
