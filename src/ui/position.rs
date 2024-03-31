//==============================================================================
//  The ui node used this logic to determine the location of the ui. Given a
//  RelativePosition and a parent bounds, you can get the bounds of a child
//  bounds.
//
//  TLDR: This is the formating logic.
//==============================================================================

use bevy::prelude::*;

use crate::common::{bounds::Bounds, value::Percent};

//==============================================================================
//          UiRelativePosition
//==============================================================================

#[derive(Reflect)]
pub enum RelativePosition {
    Align(Percent, Percent),
    Relative(Bounds),
}

impl RelativePosition {
    pub fn calc_bounds(&self, parent: &Bounds, child: &mut Bounds) {}

    pub fn create_bounds(&self, parent: &Bounds) -> Bounds {
        let mut child = Bounds::default();
        self.calc_bounds(parent, &mut child);
        child
    }

    pub fn calc_align(
        &self,
        parent: &Bounds,
        child: &mut Bounds,
        horizontal: impl Into<Percent>,
        vertical: impl Into<Percent>,
    ) {
        let horizontal: f32 = horizontal.into().to_pixels(parent.size.x);
        let vertical: f32 = vertical.into().to_pixels(parent.size.y);
    }
}
