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
