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

#[derive(Component, Reflect)]
pub struct RectComponent {
    size: Vec2,
    border_radius: Vec4,
    border_thickness: f32,
}

impl Default for RectComponent {
    fn default() -> Self {
        RectComponent {
            size: Vec2::new(100.0, 100.0),
            border_radius: Vec4::new(10.0, 10.0, 10.0, 10.0),
            border_thickness: 5.0,
        }
    }
}

impl SaikoComponent for RectComponent {
    fn render(&self, buffer: &mut SaikoBuffer) {
        buffer.push_rect(
            RectBuffer::default()
                .with_position(Vec2::new(0.0, 0.0))
                .with_size(self.size)
                .with_color(Color::MAROON)
        );
    }
}
