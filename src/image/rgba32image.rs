use super::{
    Color,
    Image
};

pub struct RGBA32Image {

}

impl Image for RGBA32Image {
    fn get_pixel(&self, pixel: (usize, usize)) -> Color {
        todo!();
    }

    fn set_pixel(&mut self, pixel: (usize, usize), color: Color) {
        todo!();
    }
}
