
//==============================================================================
//             SaikoRenderContext
//==============================================================================

use bevy::{math::{Vec2, Vec4}, render::color::Color};

use crate::{common::{bounds::Bounds, value::{Percent, Value}}, render::buffer::{BorderStyleBuffer, FillStyleBuffer, RectBuffer, SaikoBuffer}};

use super::position::RelativePosition;

pub struct SaikoRenderContext<'r> {
    buffer : &'r mut SaikoBuffer,
    bounds : Bounds
}

impl <'r> SaikoRenderContext<'r> {
    pub fn new(buffer: &'r mut SaikoBuffer, bounds: Bounds) -> Self {
        Self { buffer, bounds }
    }
    
    pub fn len(&self) -> usize {
        self.buffer.rectangles.len()
    }
}

impl <'r> SaikoRenderContextExtention for SaikoRenderContext<'r> {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }
    
    fn get_buffer(&mut self) -> &mut SaikoBuffer {
        self.buffer
    }
}

impl <'r> Drop for SaikoRenderContext<'r> {
    fn drop(&mut self) {}
}

//==============================================================================
//             SaikoRenderContextExtention Trait
//==============================================================================

pub trait SaikoRenderContextExtention: Drop {
    fn get_bounds(&self) -> &Bounds;
    
    fn get_buffer(&mut self) -> &mut SaikoBuffer;
    
    fn width(&self) -> f32 {
        self.get_bounds().size.x
    }
    
    fn height(&self) -> f32 {
        self.get_bounds().size.y
    }
    
    fn rect(&mut self) -> SaikoRenderContextRectStyler<'_> {
        SaikoRenderContextRectStyler {
            bounds: *self.get_bounds(),
            buffer: self.get_buffer(),
            border_style: BorderStyleBuffer::default(),
            fill_style: FillStyleBuffer::default(),
        }
    }
    
    fn relative(&mut self, x : f32, y : f32, width : f32, height : f32) -> SaikoRenderContext<'_> {
        let bounds = Bounds::new(
            Vec2::new(x, y),
            Vec2::new(width, height),
            self.get_bounds().z_index
        );
        
        SaikoRenderContext {
            bounds: RelativePosition::create_relative(self.get_bounds(), &bounds),
            buffer: self.get_buffer(),
        }
    }
    
    fn align(&mut self, horizontal : impl Into<Percent>, vertical : impl Into<Percent>, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        SaikoRenderContext {
            bounds: RelativePosition::create_align(self.get_bounds(), horizontal, vertical, width, height),
            buffer: self.get_buffer(),
        }
    }
    
    fn align_center(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        self.align(0.5, 0.5, width, height)
    }
    
    fn align_bottom_center(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        self.align(0.5, 0.0, width, height)
    }
    
    fn align_top_center(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        self.align(0.5, 1.0, width, height)
    }
    
    fn align_center_left(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        self.align(0.0, 0.5, width, height)
    }
    
    fn align_center_right(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        self.align(1.0, 0.5, width, height)
    }
    
    fn align_top_left(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        self.align(0.0, 1.0, width, height)
    }
    
    fn align_top_right(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        self.align(1.0, 1.0, width, height)
    }
    
    fn align_bottom_left(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        self.align(0.0, 0.0, width, height)
    }
    
    fn align_bottom_right(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'_> {
        self.align(1.0, 0.0, width, height)
    }
}

//==============================================================================
//             SaikoRenderContextRectStyler Trait
//==============================================================================

pub struct SaikoRenderContextRectStyler<'r> {
    buffer : &'r mut SaikoBuffer,
    bounds : Bounds,
    border_style : BorderStyleBuffer,
    fill_style : FillStyleBuffer
}

impl <'r> SaikoRenderContextRectStyler<'r> {

    pub fn color(mut self, color : impl Into<Color>) -> Self {
        self.fill_style.fill_color = color.into();
        self
    }
    
    pub fn border_color(mut self, color : impl Into<Color>) -> Self {
        self.border_style.border_color = color.into();
        self
    }
    
    pub fn border_width(mut self, width : f32) -> Self {
        self.border_style.border_width = width;
        self
    }
    
    pub fn border_radius(mut self, radius : impl Into<Vec4>) -> Self {
        self.border_style.border_radius = radius.into();
        self
    }
}

impl <'r> SaikoRenderContextExtention for SaikoRenderContextRectStyler<'r> {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }
    
    fn get_buffer(&mut self) -> &mut SaikoBuffer {
        self.buffer
    }
}

impl Drop for SaikoRenderContextRectStyler<'_> {
    fn drop(&mut self) {
        self.buffer.push_rect(RectBuffer {
            bound : self.bounds,
            border_style: self.border_style,
            fill_style: self.fill_style
        });
    }
}