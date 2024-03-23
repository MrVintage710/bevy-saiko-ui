//==============================================================================
//  This is a rectagle component. This is mostly used for testing the rect 
// drawer.
//==============================================================================

use bevy::prelude::*;

//==============================================================================
//          SaikoRectComponent
//==============================================================================

#[derive(Component, Reflect)]
pub struct RectComponent {
    color : Color,
}