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
    line_thickness: f32,
    line_color: Color,
}

impl Default for RectComponent {
    fn default() -> Self {
        RectComponent {
            border_radius: Vec4::new(10.0, 10.0, 10.0, 10.0),
            border_thickness: 5.0,
            background_color: Color::rgba_u8(31, 33, 49, 155),
            border_color: Color::rgba_u8(246, 92, 57, 255),
            line_thickness: 5.0,
            line_color: Color::rgba_u8(246, 92, 57, 255),
        }
    }
}

impl SaikoComponent for RectComponent {
    fn render(&self, context: &mut SaikoRenderContext<'_>) {
        context
            .line((-500.0, 0.0), (500.0, 0.0)).color(self.line_color).thickness(self.line_thickness).border_thickness(0.0)
            .line((0.0, -100.0), (0.0, 100.0)).color(self.line_color).thickness(self.line_thickness).border_thickness(0.0)
            // .rect()
            //     .border_radius(self.border_radius)
            //     .color(self.background_color)
            //     .border_color(self.border_color)
            //     .border_thickness(self.border_thickness)
            .align_center(75.0, 75.0).circle().color(self.line_color).border_thickness(self.line_thickness)
            // .relative(0.0, 0.0, 100.0, 100.0).rect()
            //     .border_radius(self.border_radius)
            //     .color(self.background_color)
            //     .border_color(self.border_color)
        ;
    }
}
