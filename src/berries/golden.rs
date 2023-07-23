use super::{
    Canvas,
    Drawable,
};

pub struct GoldenBerry;

impl Drawable for GoldenBerry {
    fn draw(&self, canvas: &mut dyn Canvas, x: usize, y: usize) {
        const BLACK: u32 = 0x000000ff;
        const   RED: u32 = 0xbc1616ff;
        const YELL0: u32 = 0x755716ff;
        const YELL1: u32 = 0x8a760fff;
        const YELL2: u32 = 0xdec02aff;
        const YELL3: u32 = 0xffee42ff;

        <Self as Drawable>::draw_sprite(canvas, x, y, 12, 15, &[
            BLACK, BLACK,     0,     0,     0, BLACK, BLACK,     0,     0,     0, BLACK, BLACK,
            BLACK, YELL1, BLACK,     0, BLACK, YELL2, YELL2, BLACK,     0, BLACK, YELL1, BLACK,
            BLACK, YELL1, YELL2, BLACK,   RED,   RED,   RED,   RED, BLACK, YELL2, YELL1, BLACK,
            BLACK, YELL0, YELL2, YELL2,   RED,   RED,   RED,   RED, YELL2, YELL2, YELL0, BLACK,
            BLACK, YELL0, YELL1, YELL2, YELL2, YELL1, YELL1, YELL2, YELL2, YELL1, YELL0, BLACK,
                0, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,     0,
                0,     0, BLACK, YELL0, YELL0, YELL0, YELL0, YELL0, YELL0, BLACK,     0,     0,
                0, BLACK, YELL1, YELL2, YELL2, YELL2, YELL2, YELL3, YELL1, YELL1, BLACK,     0,
                0, BLACK, YELL3, YELL2, YELL2, YELL3, YELL2, YELL2, YELL2, YELL3, BLACK,     0,
                0, BLACK, YELL1, YELL2, YELL2, YELL1, YELL2, YELL3, YELL2, YELL1, BLACK,     0,
                0, BLACK, YELL2, YELL1, YELL2, YELL2, YELL2, YELL1, YELL1, YELL2, BLACK,     0,
                0,     0, BLACK, YELL2, YELL1, YELL1, YELL1, YELL1, YELL2, BLACK,     0,     0,
                0,     0,     0, BLACK, YELL2, YELL1, YELL1, YELL2, BLACK,     0,     0,     0,
                0,     0,     0,     0, BLACK, YELL2, YELL2, BLACK,     0,     0,     0,     0,
                0,     0,     0,     0,     0, BLACK, BLACK,     0,     0,     0,     0,     0,
        ]);
    }
}
