use crate::{citro::c2d::Color, raw::*};
use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
};

#[repr(u32)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TextAlign {
    Left = C2D_AlignLeft,
    Right = C2D_AlignRight,
    Center = C2D_AlignCenter,
    Justified = C2D_AlignJustified,
}

pub struct TextRender {
    c2d_text: C2D_Text,
    scale_x: f32,
    scale_y: f32,

    color: Color,
    has_color: bool,

    align: TextAlign,
}

impl Drop for TextRender {
    fn drop(&mut self) {
        unsafe {
            C2D_TextBufDelete(self.c2d_text.buf);
        }
    }
}

impl Default for TextRender {
    fn default() -> Self {
        Self::new()
    }
}

impl TextRender {
    pub fn align(&self) -> TextAlign {
        self.align
    }

    pub fn set_align(&mut self, align: TextAlign) {
        self.align = align;
    }

    pub fn color(&self) -> Option<Color> {
        Some(self.color).filter(|_| self.has_color)
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.has_color = color.is_some();
        if let Some(color) = color {
            self.color = color;
        }
    }
}

impl TextRender {
    pub const DEFAULT_CAPACITY: usize = 200;

    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let buf = unsafe { C2D_TextBufNew(capacity as size_t) };
        assert!(
            !buf.is_null(),
            "Cannot allocate TextRender buffer of capacity {}",
            capacity
        );

        TextRender {
            c2d_text: C2D_Text {
                buf,
                begin: 0,
                end: 0,
                width: 0.0,
                lines: 0,
                words: 0,
                font: std::ptr::null_mut(),
            },
            scale_x: 1.0,
            scale_y: 1.0,

            color: Color::new_alpha(0, 0, 0, 0),
            has_color: false,

            align: TextAlign::Left,
        }
    }

    pub fn append_c_str(&mut self, s: &CStr) -> usize {
        unsafe {
            let ptr = s.as_ptr();
            let processed = C2D_TextParse(&mut self.c2d_text, self.c2d_text.buf, ptr);
            assert!(
                !processed.is_null(),
                "Could not append {:?} to TextRender",
                s
            );
            processed.offset_from(ptr) as usize
        }
    }

    pub fn append_str(&mut self, s: &str) -> usize {
        let c = CString::new(s).unwrap();
        self.append_c_str(c.as_c_str())
    }

    pub fn set_str(&mut self, s: &str) -> usize {
        self.clear();
        self.append_str(s)
    }

    pub fn clear(&mut self) {
        unsafe {
            C2D_TextBufClear(self.c2d_text.buf);
            self.c2d_text = C2D_Text{
                begin: 0,
                end: 0,
                width: 0.0,
                lines: 0,
                words: 0,
                ..self.c2d_text
            };
        }
    }

    pub fn dimensions(&self) -> (f32, f32) {
        unsafe {
            let mut x = MaybeUninit::uninit();
            let mut y = MaybeUninit::uninit();

            C2D_TextGetDimensions(
                &self.c2d_text,
                self.scale_x,
                self.scale_y,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );

            (x.assume_init(), y.assume_init())
        }
    }

    pub fn draw(&self, x: f32, y: f32, z: f32) {
        unsafe {
            if self.has_color {
                C2D_DrawText(
                    &self.c2d_text,
                    self.align as u32 | C2D_WithColor,
                    x,
                    y,
                    z,
                    self.scale_x,
                    self.scale_y,
                    self.color.0,
                )
            } else {
                C2D_DrawText(
                    &self.c2d_text,
                    self.align as u32,
                    x,
                    y,
                    z,
                    self.scale_x,
                    self.scale_y,
                )
            }
        }
    }
}
