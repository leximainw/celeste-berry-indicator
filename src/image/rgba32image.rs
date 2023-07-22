use super::{
    Color,
    Image
};

pub struct RGBA32Image {

}

impl Image for RGBA32Image {
    fn get_pixel(&self, pixel: (i32, i32)) -> Color {
        todo!();
    }

    fn set_pixel(&mut self, pixel: (i32, i32), color: Color) {
        todo!();
    }
}
