use super::RGBA32Image;

pub trait Image {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn get_pixel(&self, pixel: (usize, usize)) -> Color;
    fn set_pixel(&mut self, pixel: (usize, usize), color: Color);
    fn get_pixel_index(&self, index: usize) -> Color;
    fn set_pixel_index(&mut self, index: usize, color: Color);

    fn to_rgba32_image(self: Box<Self>) -> RGBA32Image;
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u32,
    g: u32,
    b: u32,
    a: u32,
}

impl Color {
    pub const zero: Color = Color{
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r as u32 * 0x01010101,
            g: g as u32 * 0x01010101,
            b: b as u32 * 0x01010101,
            a: a as u32 * 0x01010101,
        }
    }

    pub fn from_rgba32(color: u32) -> Color {
        Color {
            r: (color >> 24 & 255) * 0x01010101,
            g: (color >> 16 & 255) * 0x01010101,
            b: (color >> 8 & 255) * 0x01010101,
            a: (color & 255) * 0x01010101,
        }
    }

    pub fn to_rgba32(color: Color) -> u32 {
        (color.r >> 24 << 24) | (color.g >> 24 << 16) | (color.b >> 24 << 8) | (color.a >> 24)
    }
}
