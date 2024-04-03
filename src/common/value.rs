//==============================================================================
//  Saiko ui can be layed out with many different typpes of values such as
//  pixels or percentages. Those values are defined here.
//==============================================================================

use std::ops::{Deref, DerefMut};

use bevy::{ecs::reflect, prelude::*};

//==============================================================================
//          Value Enum
//==============================================================================

/// Value is an enum that is used to determine the type of value that is being used.
#[derive(Reflect, Clone, Copy)]
#[reflect(Default)]
pub enum Value {
    /// Pixel value with support for subpixel values.
    #[reflect(default)]
    Px(f32),
    /// Percentage value.
    #[reflect(default)]
    Percent(Percent),
}

impl Default for Value {
    fn default() -> Self {
        Value::Px(0.0)
    }
}

impl Value {
    pub fn to_pixels(&self, reference: f32) -> f32 {
        match self {
            Value::Px(px) => *px,
            Value::Percent(percent) => percent.to_pixels(reference),
        }
    }
}

impl From<Percent> for Value {
    fn from(p: Percent) -> Self {
        Value::Percent(p)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Px(f)
    }
}

//==============================================================================
//          Percent
//==============================================================================

#[derive(Reflect, Default, Clone, Copy)]
#[reflect(Default)]
pub struct Percent(f32);

impl Percent {
    pub fn new(value: f32) -> Self {
        Percent(value.clamp(0.0, 1.0))
    }

    pub fn to_pixels(&self, reference: f32) -> f32 {
        reference * self.0
    }

    pub fn set(&mut self, value: f32) {
        self.0 = value.clamp(0.0, 1.0);
    }
}

impl Deref for Percent {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Percent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0 = self.0.clamp(0.0, 1.0);
        &mut self.0
    }
}

impl From<f32> for Percent {
    fn from(f: f32) -> Self {
        Percent::new(f)
    }
}

impl From<Percent> for f32 {
    fn from(p: Percent) -> Self {
        p.0
    }
}
