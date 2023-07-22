use super::{
    Color,
    RGBA32Image,
};

pub trait Image {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn get_pixel(&self, pixel: (usize, usize)) -> Color;
    fn set_pixel(&mut self, pixel: (usize, usize), color: Color);
    fn get_pixel_index(&self, index: usize) -> Color;
    fn set_pixel_index(&mut self, index: usize, color: Color);

    fn to_rgba32_image(self: Box<Self>) -> RGBA32Image;
}
