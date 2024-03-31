//==============================================================================
//  This is a rectagle component. This is mostly used for testing the rect
// drawer.
//==============================================================================

use bevy::prelude::*;

use crate::render::buffer::{RectBuffer, SaikoBuffer};

use super::SaikoComponent;

//==============================================================================
//          SaikoRectComponent
//==============================================================================

#[derive(Component, Reflect, Default)]
pub struct RectComponent {
    size: Vec2,
}

impl SaikoComponent for RectComponent {
    fn render(&self, buffer: &mut SaikoBuffer) {
        buffer.push_rect(
            RectBuffer::default()
                .with_position(Vec3::new(0.0, 0.0, 0.0))
                .with_size(Vec2::new(10.0, 10.0))
                .with_color((0.0, 0.0, 1.0, 0.5))
                .with_corners(Vec4::new(0.0, 0.0, 0.0, 0.0)),
        );
    }
}
