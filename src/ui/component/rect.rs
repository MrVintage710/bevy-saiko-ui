//==============================================================================
//  This is a rectagle component. This is mostly used for testing the rect
// drawer.
//==============================================================================

use bevy::prelude::*;

use crate::{render::buffer::{RectBuffer, SaikoBuffer}, ui::context::{SaikoRenderContext, SaikoRenderContextExtention}};

use super::SaikoComponent;

//==============================================================================
//          SaikoRectComponent
//==============================================================================

#[derive(Component, Reflect)]
pub struct RectComponent {
    size: Vec2,
    border_radius: Vec4,
    border_thickness: f32,
    background_color: Color,
    border_color: Color,
    position: Vec2,
}

impl Default for RectComponent {
    fn default() -> Self {
        RectComponent {
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(100.0, 100.0),
            border_radius: Vec4::new(10.0, 10.0, 10.0, 10.0),
            border_thickness: 5.0,
            background_color: Color::rgba_u8(31, 33, 49, 255),
            border_color: Color::rgba_u8(246, 92, 57, 255),
        }
    }
}

impl SaikoComponent for RectComponent {
    fn render(&self, context: &mut SaikoRenderContext<'_>) {
        context.rect()
            .border_radius(self.border_radius)
            .color(self.background_color)
            .border_color(self.border_color)
            .border_width(self.border_thickness)
        ;
        
        // buffer.push_rect(
        //     RectBuffer::default()
        //         .with_position(self.position)
        //         .with_size(self.size)
        //         .with_color(self.background_color)
        //         .with_border_radius(self.border_radius)
        //         .with_border_width(self.border_thickness)
        //         .with_border_color(self.border_color)
        // );
    }
}
