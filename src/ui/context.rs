use std::{rc::Rc, sync::RwLock};

use bevy::{asset::{Assets, Handle}, math::{Vec2, Vec4}, render::color::Color, text::Font};

use crate::{common::{bounds::Bounds, value::{Percent, Value}}, render::{buffer::{BorderStyleBuffer, FillStyleBuffer, RectBuffer, SaikoBuffer}, font::sdf::{SaikoFontSdf, DEFAULT_FONT}}};

use super::position::RelativePosition;

//==============================================================================
//             SaikoInnerContext
//==============================================================================

pub struct SaikoInnerContext<'r> {
    buffer : &'r mut SaikoBuffer,
    fonts : &'r Assets<SaikoFontSdf>,
}

//==============================================================================
//             SaikoRenderContext
//==============================================================================

pub struct SaikoRenderContext<'r> {
    inner : Rc<RwLock<SaikoInnerContext<'r>>>,
    bounds : Bounds
}

impl <'r> SaikoRenderContext<'r> {
    pub fn new(buffer: &'r mut SaikoBuffer, fonts: &'r Assets<SaikoFontSdf>, bounds: Bounds) -> Self {
        Self { inner : Rc::new(RwLock::new(SaikoInnerContext { buffer, fonts })) , bounds }
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().buffer.rectangles.len()
    }
}

impl <'r> SaikoRenderContextExtention<'r> for SaikoRenderContext<'r> {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn get_inner(&mut self) -> Rc<RwLock<SaikoInnerContext<'r>>> {
        self.inner.clone()
    }
}

impl <'r> Drop for SaikoRenderContext<'r> {
    fn drop(&mut self) {}
}

//==============================================================================
//             SaikoRenderContextExtention Trait
//==============================================================================

pub trait SaikoRenderContextExtention<'r> : Drop {
    fn get_inner(&mut self) -> Rc<RwLock<SaikoInnerContext<'r>>>;
    
    fn get_bounds(&self) -> &Bounds;
    
    fn width(&self) -> f32 {
        self.get_bounds().size.x
    }
    
    fn height(&self) -> f32 {
        self.get_bounds().size.y
    }
    
    fn rect(&mut self) -> SaikoRenderContextRectStyler<'r> {
        SaikoRenderContextRectStyler {
            bounds: *self.get_bounds(),
            inner : self.get_inner(),
            border_style: BorderStyleBuffer::default(),
            fill_style: FillStyleBuffer::default(),
        }
    }
    
    fn text(&mut self, text : &str) -> SaikoRenderContextTextStyler<'r> {
        SaikoRenderContextTextStyler {
            bounds: *self.get_bounds(),
            inner : self.get_inner(),
            text: text.to_string(),
            font: None,
        }
    }
    
    fn relative(&mut self, x : f32, y : f32, width : f32, height : f32) -> SaikoRenderContext<'r> {
        let bounds = Bounds::new(
            Vec2::new(x, y),
            Vec2::new(width, height),
            self.get_bounds().z_index
        );
        
        SaikoRenderContext {
            bounds: RelativePosition::create_relative(self.get_bounds(), &bounds),
            inner: self.get_inner(),
        }
    }
    
    fn align(&mut self, horizontal : impl Into<Percent>, vertical : impl Into<Percent>, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        SaikoRenderContext {
            bounds: RelativePosition::create_align(self.get_bounds(), horizontal, vertical, width, height),
            inner: self.get_inner(),
        }
    }
    
    fn align_center(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        self.align(0.5, 0.5, width, height)
    }
    
    fn align_bottom_center(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        self.align(0.5, 0.0, width, height)
    }
    
    fn align_top_center(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        self.align(0.5, 1.0, width, height)
    }
    
    fn align_center_left(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        self.align(0.0, 0.5, width, height)
    }
    
    fn align_center_right(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        self.align(1.0, 0.5, width, height)
    }
    
    fn align_top_left(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        self.align(0.0, 1.0, width, height)
    }
    
    fn align_top_right(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        self.align(1.0, 1.0, width, height)
    }
    
    fn align_bottom_left(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        self.align(0.0, 0.0, width, height)
    }
    
    fn align_bottom_right(&mut self, width : impl Into<Value>, height : impl Into<Value>) -> SaikoRenderContext<'r> {
        self.align(1.0, 0.0, width, height)
    }
}

//==============================================================================
//             SaikoRenderContextRectStyler
//==============================================================================

pub struct SaikoRenderContextRectStyler<'r> {
    inner : Rc<RwLock<SaikoInnerContext<'r>>>,
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

impl <'r> SaikoRenderContextExtention<'r> for SaikoRenderContextRectStyler<'r> {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn get_inner(&mut self) -> Rc<RwLock<SaikoInnerContext<'r>>> {
        self.inner.clone()
    }
}

impl Drop for SaikoRenderContextRectStyler<'_> {
    fn drop(&mut self) {
        let mut inner = self.inner.write().unwrap();
        inner.buffer.push_rect(RectBuffer {
            bound : self.bounds,
            border_style: self.border_style,
            fill_style: self.fill_style
        });
    }
}

//==============================================================================
//             SaikoRenderContextTextStyler
//==============================================================================

pub struct SaikoRenderContextTextStyler<'r> {
    inner : Rc<RwLock<SaikoInnerContext<'r>>>,
    text : String,
    font : Option<Handle<SaikoFontSdf>>,
    bounds : Bounds,
}

impl <'r> SaikoRenderContextTextStyler<'r> {
    pub fn font(mut self, font : Handle<SaikoFontSdf>) -> Self {
        self.font = Some(font);
        self
    }
}

impl <'r> SaikoRenderContextExtention<'r> for SaikoRenderContextTextStyler<'r> {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }
    
    fn get_inner(&mut self) -> Rc<RwLock<SaikoInnerContext<'r>>> {
        self.inner.clone()
    }
}

impl Drop for SaikoRenderContextTextStyler<'_> {
    fn drop(&mut self) {
        let mut inner = self.inner.write().unwrap();
        let font_handle = self.font.clone().unwrap_or(DEFAULT_FONT);
        let font = inner.fonts.get(font_handle).expect("While rendering text, tried to render a font that doesn't exist.");
        
        
    }
}