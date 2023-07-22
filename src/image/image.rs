pub trait Image {
    fn get_pixel(&self, pixel: (usize, usize)) -> Color;
    fn set_pixel(&mut self, pixel: (usize, usize), color: Color);
}

pub struct Color {
    r: u32,
    g: u32,
    b: u32,
    a: u32,
}

impl Color {
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
