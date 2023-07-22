pub trait Image {
    fn get_pixel(pixel: (i32, i32)) -> Color;
    fn set_pixel(pixel: (i32, i32), color: Color);
}

pub struct Color {
    r: i32,
    g: i32,
    b: i32,
    a: i32,
}
