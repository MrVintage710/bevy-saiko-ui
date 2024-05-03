//==============================================================================
//  This is a rectagle component. This is mostly used for testing the rect
// drawer.
//==============================================================================

use bevy::prelude::*;

use crate::ui::context::{SaikoRenderContext, SaikoRenderContextExtention};

use super::SaikoComponent;

//==============================================================================
//          SaikoRectComponent
//==============================================================================

#[derive(Component, Reflect)]
pub struct RectComponent {
    border_radius: Vec4,
    border_thickness: f32,
    background_color: Color,
    border_color: Color,
}

impl Default for RectComponent {
    fn default() -> Self {
        RectComponent {
            border_radius: Vec4::new(10.0, 10.0, 10.0, 10.0),
            border_thickness: 5.0,
            background_color: Color::rgba_u8(31, 33, 49, 255),
            border_color: Color::rgba_u8(246, 92, 57, 255),
        }
    }
}

impl SaikoComponent for RectComponent {
    fn render(&self, context: &mut SaikoRenderContext<'_>) {
        context
            .line((0.0, -100.0), (0.0, 100.0)).color(Color::RED.with_a(0.5)).thickness(10.0).border_thickness(4.0)
            .line((-100.0, 0.0), (100.0, 0.0)).color(Color::RED)
            .rect()
                .border_radius(self.border_radius)
                .color(self.background_color)
                .border_color(self.border_color)
                .border_thickness(self.border_thickness)
            // .relative(100.0 + self.border_thickness / 2.0, 0.0, 100.0, 100.0).rect()
            //     .border_radius(self.border_radius)
            //     .color(self.background_color)
            //     .border_color(self.border_color)
            //     .border_width(self.border_thickness)
        ;
    }
}
