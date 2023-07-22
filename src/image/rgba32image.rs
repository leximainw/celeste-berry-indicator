use super::{
    Color,
    Image
};

#[derive(Debug)]
pub struct RGBA32Image {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl RGBA32Image {
    pub fn new(width: usize, height: usize) -> RGBA32Image {
        RGBA32Image{
            width,
            height,
            pixels: vec![0; width * height],
        }
    }
}

impl Image for RGBA32Image {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_pixel(&self, pixel: (usize, usize)) -> Color {
        Color::from_srgba32(self.pixels[pixel.0 + pixel.1 * self.width])
    }

    fn set_pixel(&mut self, pixel: (usize, usize), color: Color) {
        self.pixels[pixel.0 + pixel.1 * self.width] = Color::to_srgba32(color);
    }

    fn get_pixel_index(&self, index: usize) -> Color {
        Color::from_srgba32(self.pixels[index])
    }

    fn set_pixel_index(&mut self, index: usize, color: Color) {
        self.pixels[index] = Color::to_srgba32(color)
    }

    fn to_rgba32_image(self: Box<Self>) -> Self {
        *self
    }
}
