//==============================================================================
//  The ui node used this logic to determine the location of the ui. Given a
//  RelativePosition and a parent bounds, you can get the bounds of a child
//  bounds.
//
//  TLDR: This is the formating logic.
//==============================================================================

use bevy::prelude::*;

use crate::common::{bounds::Bounds, value::{Percent, Value}};

//==============================================================================
//          UiRelativePosition
//==============================================================================

#[derive(Reflect)]
#[reflect(Default)]
pub enum RelativePosition {
    #[reflect(default)]
    Align(Percent, Percent, Value, Value),
    #[reflect(default)]
    Relative(Bounds),
}

impl Default for RelativePosition {
    fn default() -> Self {
        RelativePosition::Relative(Bounds::default())
    }
}

impl RelativePosition {
    pub fn calc_bounds(&self, parent: &Bounds, child: &mut Bounds) {
        match self {
            RelativePosition::Align(horizontal, vertical, width, height) => 
                Self::calc_align(parent, child, *horizontal, *vertical, *width, *height),
            RelativePosition::Relative(bounds) => 
                Self::calc_relative(parent, child, bounds),
        }
    }

    pub fn create_bounds(&self, parent: &Bounds) -> Bounds {
        let mut child = Bounds::default();
        self.calc_bounds(parent, &mut child);
        child
    }

    pub fn calc_align(
        parent: &Bounds,
        child: &mut Bounds,
        horizontal: impl Into<Percent>,
        vertical: impl Into<Percent>,
        width: impl Into<Value>,
        height: impl Into<Value>,
    ) {
        println!("Parent : {:?}", parent);
        
        let width = width.into().to_pixels(parent.size.x);
        let height = height.into().to_pixels(parent.size.y);
        let adjusted_width = parent.size.x - width;
        let adjusted_height = parent.size.y - height;
        let horizontal = horizontal.into().to_pixels(adjusted_width);
        let vertical = vertical.into().to_pixels(adjusted_height);
        
        let x = parent.center.x - (adjusted_width) + horizontal * 2.0;
        let y = parent.center.y - (adjusted_height) + vertical * 2.0;
        
        child.center = Vec2::new(x, y);
        child.size = Vec2::new(width, height);
    }
    
    pub fn calc_relative(parent: &Bounds, child: &mut Bounds, bounds: &Bounds) {
        child.center = parent.center + bounds.center;
        child.size = bounds.size;
    }
    
    pub fn create_relative(parent: &Bounds, bounds: &Bounds) -> Bounds {
        let mut child = Bounds::default();
        Self::calc_relative(parent, &mut child, bounds);
        child
    }
}
