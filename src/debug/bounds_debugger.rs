
use bevy::prelude::*;

use crate::{common::bounds::Bounds, ui::node::SaikoNode};

//==============================================================================
//             Bounds Debugger plugin
//==============================================================================

pub(crate) struct SaikoBoundsDebuggerPlugin;

impl Plugin for SaikoBoundsDebuggerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, debug_bounds);
    }
}

//==============================================================================
//             Outline Bounds
//==============================================================================

fn debug_bounds(
    mut gizmos : Gizmos,
    nodes : Query<&SaikoNode>
) {
    for node in nodes.iter() {
        gizmo_bounds(&mut gizmos, node, Color::ALICE_BLUE);
    }
}

pub fn gizmo_bounds(
    gizmos : &mut Gizmos,
    bounds : &SaikoNode,
    color : Color
) {
    let bounds = bounds.bounds();
    
    let horizontal_offset = Vec2::new(10.0, 0.0);
    let vertical_offset = Vec2::new(0.0, 10.0);
    
    gizmos.line_2d(bounds.center + vertical_offset, bounds.center - vertical_offset, color);
    gizmos.line_2d(bounds.center + horizontal_offset, bounds.center - horizontal_offset, color);
    gizmos.circle_2d(bounds.center, 3.0, color);
}