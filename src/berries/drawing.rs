use crate::image::{
    Color,
    Image,
};

pub trait Canvas {
    fn get_pixel(&self, x: usize, y: usize) -> u32;
    fn set_pixel(&mut self, x: usize, y: usize, value: u32);
}

pub trait Drawable {
    fn draw(&self, canvas: &mut dyn Canvas, x: usize, y: usize);

    fn draw_sprite(canvas: &mut dyn Canvas, x: usize, y: usize, width: usize, height: usize, pixels: &[u32]) {
        for u in 0..width {
            for v in 0..height {
                let pixel = pixels[u + v * width];
                if pixel != 0 {
                    canvas.set_pixel(x + u, y + v, pixel);
                }
            }
        }
    }
}

pub struct OpaqueCanvas<'a> {
    image: &'a mut dyn Image,
}

impl OpaqueCanvas<'_> {
    pub fn from_image(image: &mut dyn Image) -> OpaqueCanvas {
        OpaqueCanvas{
            image,
        }
    }
}

impl Canvas for OpaqueCanvas<'_> {
    fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.image.get_pixel((x, y)).to_srgba32()
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        self.image.set_pixel((x, y), Color::from_srgba32(value));
    }
}

pub struct FadedCanvas<'a> {
    image: &'a mut dyn Image,
}

impl FadedCanvas<'_> {
    pub fn from_image(image: &mut dyn Image) -> FadedCanvas {
        FadedCanvas{
            image,
        }
    }
}

impl Canvas for FadedCanvas<'_> {
    fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.image.get_pixel((x, y)).to_srgba32()
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        let value = Color::from_srgba32(value).desaturate();
        let value = value.set_alpha(value.get_alpha() / 2);
        self.image.set_pixel((x, y), Color::alpha_over(value, self.image.get_pixel((x, y))));
    }
}
