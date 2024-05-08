use std::{rc::Rc, sync::RwLock};

use bevy::{asset::{Assets, Handle}, math::{Vec2, Vec4}, render::color::Color, text::Font};
use cosmic_text::Buffer;

use crate::{common::{bounds::Bounds, value::{Percent, Value}, TextHorizontalAlign, TextVerticalAlign}, render::{buffer::{BorderStyleBuffer, FillStyleBuffer, LineBuffer, LineStyleBuffer, SaikoBuffer, SimpleShapeBuffer}, font::sdf::{SaikoFontSdf, DEFAULT_FONT}}};

use super::position::RelativePosition;

//==============================================================================
//             SaikoInnerContext
//==============================================================================

pub struct SaikoInnerContext<'r> {
    buffer : &'r mut SaikoBuffer,
    fonts : &'r Assets<SaikoFontSdf>,
    should_debug : bool
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
        Self { inner : Rc::new(RwLock::new(SaikoInnerContext { buffer, fonts, should_debug : false})) , bounds }
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().buffer.rectangles.len()
    }
}

impl <'r> SaikoRenderContextExtention<'r> for SaikoRenderContext<'r> {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn get_inner(&self) -> Rc<RwLock<SaikoInnerContext<'r>>> {
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
    fn get_inner(&self) -> Rc<RwLock<SaikoInnerContext<'r>>>;
    
    fn get_bounds(&self) -> &Bounds;
    
    fn get_next_bounds(&self) -> Bounds {
        let mut bounds = *self.get_bounds();
        bounds.z_index += 1;
        bounds
    }
    
    fn width(&self) -> f32 {
        self.get_bounds().size.x
    }
    
    fn height(&self) -> f32 {
        self.get_bounds().size.y
    }
    
    fn rect(&mut self) -> SaikoRenderContextRectStyler<'r> {
        let inner = self.get_inner();
        inner.write().unwrap().should_debug = false;
        SaikoRenderContextRectStyler {
            bounds: self.get_next_bounds(),
            inner,
            border_style: BorderStyleBuffer::default(),
            fill_style: FillStyleBuffer::default(),
        }
    }
    
    fn circle(&mut self) -> SaikoRenderContextCircleStyler<'r> {
        let inner = self.get_inner();
        inner.write().unwrap().should_debug = false;
        SaikoRenderContextCircleStyler {
            bounds: self.get_next_bounds(),
            inner,
            border_style: BorderStyleBuffer::default(),
            fill_style: FillStyleBuffer::default(),
        }
    }
    
    fn line(&mut self, a : impl Into<Vec2>, b : impl Into<Vec2>) -> SaikoRenderContextLineStyler<'r> {
        let inner = self.get_inner();
        inner.write().unwrap().should_debug = false;
        let mut border_style = BorderStyleBuffer::default();
        border_style.border_width = 0.0;
        SaikoRenderContextLineStyler {
            bounds: self.get_next_bounds(),
            inner,
            a : a.into() + self.get_bounds().center,
            b : b.into() + self.get_bounds().center,
            line_style: LineStyleBuffer::default(),
            border_style,
            fill_style: FillStyleBuffer::default(),
        }
    }
    
    fn text(&mut self, text : &str) -> SaikoRenderContextTextStyler<'r> {
        let inner = self.get_inner();
        inner.write().unwrap().should_debug = false;
        SaikoRenderContextTextStyler {
            bounds: self.get_next_bounds(),
            inner,
            text: text.to_string(),
            horizontal_align: TextHorizontalAlign::Left,
            vertical_align : TextVerticalAlign::Top,
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
            bounds: RelativePosition::create_align(&self.get_next_bounds(), horizontal, vertical, width, height),
            inner: self.get_inner(),
        }
    }
    
    fn debug(&mut self) {
        if let Ok(mut inner) = self.get_inner().try_write() {
            inner.should_debug = true;
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
    
    pub fn border_thickness(mut self, width : f32) -> Self {
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

    fn get_inner(&self) -> Rc<RwLock<SaikoInnerContext<'r>>> {
        self.inner.clone()
    }
}

impl Drop for SaikoRenderContextRectStyler<'_> {
    fn drop(&mut self) {
        if self.inner.read().unwrap().should_debug {
            self
                .line((-25.0, 0.0), (25.0, 0.0)).color(Color::RED)
                .line((0.0, -25.0), (0.0, 25.0)).color(Color::RED)
                .align_center(10.0, 10.0).circle().color(Color::RED).border_thickness(0.0)
            ;
        }
        
        let mut inner = self.inner.write().unwrap();
        inner.buffer.push_rect(SimpleShapeBuffer {
            bound : self.bounds,
            border_style: self.border_style,
            fill_style: self.fill_style
        });
        
    }
}

//==============================================================================
//             SaikoRenderContextRectStyler
//==============================================================================

pub struct SaikoRenderContextCircleStyler<'r> {
    inner : Rc<RwLock<SaikoInnerContext<'r>>>,
    bounds : Bounds,
    border_style : BorderStyleBuffer,
    fill_style : FillStyleBuffer
}

impl <'r> SaikoRenderContextCircleStyler<'r> {

    pub fn color(mut self, color : impl Into<Color>) -> Self {
        self.fill_style.fill_color = color.into();
        self
    }
    
    pub fn border_color(mut self, color : impl Into<Color>) -> Self {
        self.border_style.border_color = color.into();
        self
    }
    
    pub fn border_thickness(mut self, width : f32) -> Self {
        self.border_style.border_width = width;
        self
    }
    
    pub fn border_radius(mut self, radius : impl Into<Vec4>) -> Self {
        self.border_style.border_radius = radius.into();
        self
    }
}

impl <'r> SaikoRenderContextExtention<'r> for SaikoRenderContextCircleStyler<'r> {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn get_inner(&self) -> Rc<RwLock<SaikoInnerContext<'r>>> {
        self.inner.clone()
    }
}

impl Drop for SaikoRenderContextCircleStyler<'_> {
    fn drop(&mut self) {
        let mut inner = self.inner.write().unwrap();
        inner.buffer.push_circle(SimpleShapeBuffer {
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
    horizontal_align : TextHorizontalAlign,
    vertical_align : TextVerticalAlign
}

impl <'r> SaikoRenderContextTextStyler<'r> {
    pub fn font(mut self, font : Handle<SaikoFontSdf>) -> Self {
        self.font = Some(font);
        self
    }
    
    pub fn horizontal_align(mut self, align : TextHorizontalAlign) -> Self {
        self.horizontal_align = align;
        self
    }
    
    pub fn vertical_align(mut self, align : TextVerticalAlign) -> Self {
        self.vertical_align = align;
        self
    }
}

impl <'r> SaikoRenderContextExtention<'r> for SaikoRenderContextTextStyler<'r> {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }
    
    fn get_inner(&self) -> Rc<RwLock<SaikoInnerContext<'r>>> {
        self.inner.clone()
    }
}

impl Drop for SaikoRenderContextTextStyler<'_> {
    fn drop(&mut self) {
        // if self.inner.read().unwrap().should_debug {
        //     self
                
        //     ;
        // }
        
        let mut inner = self.inner.write().unwrap();
        let font_handle = self.font.clone().unwrap_or(DEFAULT_FONT);
        let font = inner.fonts.get(font_handle).expect("While rendering text, tried to render a font that doesn't exist.");
        
        // let buffer = Buffer::new(font_system, metrics)
    }
}

//==============================================================================
//             SaikoRenderContextTextStyler
//==============================================================================

pub struct SaikoRenderContextLineStyler<'r> {
    inner : Rc<RwLock<SaikoInnerContext<'r>>>,
    bounds : Bounds,
    a : Vec2,
    b : Vec2,
    line_style : LineStyleBuffer,
    border_style : BorderStyleBuffer,
    fill_style : FillStyleBuffer
}

impl <'r> SaikoRenderContextLineStyler<'r> {
    pub fn thickness(mut self, thickness : f32) -> Self {
        self.line_style.thickness = thickness;
        self
    }
    
    pub fn color(mut self, color : impl Into<Color>) -> Self {
        self.fill_style.fill_color = color.into();
        self
    }
    
    pub fn border_color(mut self, color : impl Into<Color>) -> Self {
        self.border_style.border_color = color.into();
        self
    }
    
    pub fn border_thickness(mut self, thickness : f32) -> Self {
        self.border_style.border_width = thickness;
        self
    }
    
    pub fn border_radius(mut self, radius : impl Into<Vec4>) -> Self {
        self.border_style.border_radius = radius.into();
        self
    }
}

impl <'r> SaikoRenderContextExtention<'r> for SaikoRenderContextLineStyler<'r> {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }
    
    fn get_inner(&self) -> Rc<RwLock<SaikoInnerContext<'r>>> {
        self.inner.clone()
    }
}

impl Drop for SaikoRenderContextLineStyler<'_> {
    fn drop(&mut self) {
        let mut inner = self.inner.write().unwrap();
        inner.buffer.push_line(LineBuffer {
            bounds : Bounds::new(self.a, self.b, self.bounds.z_index),
            line_style : self.line_style,
            border_style: self.border_style,
            fill_style: self.fill_style
        });
    }
}