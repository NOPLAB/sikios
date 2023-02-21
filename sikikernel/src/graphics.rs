use crate::ascii_font::FONTS;
use lib::{FrameBufferInfo, ModeInfo, PixelFormat};

#[derive(Debug, Copy, Clone)]
pub struct Color(pub u32, pub u32, pub u32);

#[derive(Debug, Copy, Clone)]
pub struct Graphics {
    pub frame_buffer_info: FrameBufferInfo,
    pub mode_info: ModeInfo,
}

impl Graphics {
    pub fn get_resolve(&mut self) -> (u32, u32) {
        unsafe { ((self.mode_info).hor_res, (self.mode_info).ver_res) }
    }

    pub fn pixel_position(&mut self, x: u32, y: u32) -> usize {
        unsafe { (((self.mode_info).hor_res * y + x) * 4) as usize }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, pixel_color: Color) {
        let (width, height) = self.get_resolve();
        if !(x <= width && y <= height) {
            return;
        }

        unsafe {
            let i = self.pixel_position(x, y);
            let buffer = core::slice::from_raw_parts_mut(
                (self.frame_buffer_info).fb,
                (self.frame_buffer_info).size,
            );
            if (self.mode_info).format == PixelFormat::Rgb {
                buffer[i] = pixel_color.0 as u8;
                buffer[i + 1] = pixel_color.1 as u8;
                buffer[i + 2] = pixel_color.2 as u8;
            }
            if (self.mode_info).format == PixelFormat::Bgr {
                buffer[i] = pixel_color.2 as u8;
                buffer[i + 1] = pixel_color.1 as u8;
                buffer[i + 2] = pixel_color.0 as u8;
            }
        }
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, pixel_color: Color) {
        for i in 0..width {
            for j in 0..height {
                self.draw_pixel(i + x, j + y, pixel_color);
            }
        }
    }

    pub fn draw_font(&mut self, x: u32, y: u32, c: char, pixel_color: Color) {
        if c as u32 > 0x7f {
            return;
        }
        let font: [u8; 16] = FONTS[c as usize];
        for (dy, line) in font.iter().enumerate() {
            for dx in 0..8 {
                if (line << dx) & 0x80 != 0 {
                    self.draw_pixel(x + dx, y + dy as u32, pixel_color);
                }
            }
        }
    }

    pub fn draw_fonts(&mut self, mut x: u32, mut y: u32, str: &str, pixel_color: Color) {
        let first_x = x;
        let (width, height) = self.get_resolve();
        for c in str.chars() {
            self.draw_font(x, y, c, pixel_color);
            x += 8;
            if x > width {
                x = first_x;
                y += 20;
            }
            if y > height {
                return;
            }
        }
    }
}
