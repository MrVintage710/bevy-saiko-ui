
//==============================================================================
//             SaikoRenderContext
//==============================================================================

use bevy::{math::{Vec2, Vec4}, render::color::Color, utils::OnDrop};

use crate::{common::bounds::Bounds, render::buffer::{BorderStyleBuffer, FillStyleBuffer, RectBuffer, SaikoBuffer}};

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
    
    fn rect(&mut self) -> SaikoRenderContextRectStyler<'_> {
        SaikoRenderContextRectStyler {
            bounds: *self.get_bounds(),
            buffer: self.get_buffer(),
            border_style: BorderStyleBuffer::default(),
            fill_style: FillStyleBuffer::default(),
        }
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