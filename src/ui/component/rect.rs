//==============================================================================
//  This is a rectagle component. This is mostly used for testing the rect 
// drawer.
//==============================================================================

use bevy::prelude::*;

use crate::render::buffer::SaikoBuffer;

use super::SaikoComponent;

//==============================================================================
//          SaikoRectComponent
//==============================================================================

#[derive(Component, Reflect)]
pub struct RectComponent {
    color : Color,
}

impl SaikoComponent for RectComponent {
    fn render(&self, buffer : &mut SaikoBuffer) {
        buffer.push_rect(rect)
    }
}