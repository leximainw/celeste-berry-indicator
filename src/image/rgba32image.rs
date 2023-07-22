use super::{
    Color,
    Image
};

pub struct RGBA32Image {
    width: usize,
    height: usize,
    pixels: [u32],
}

impl Image for RGBA32Image {
    fn get_pixel(&self, pixel: (usize, usize)) -> Color {
        Color::from_rgba32(self.pixels[pixel.0 + pixel.1 * self.width])
    }

    fn set_pixel(&mut self, pixel: (usize, usize), color: Color) {
        self.pixels[pixel.0 + pixel.1 * self.width] = Color::to_rgba32(color);
    }
}
