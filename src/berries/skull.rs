use super::{
    Canvas,
    Drawable,
};

pub struct Skull;

impl Drawable for Skull {
    fn draw(&self, canvas: &mut dyn Canvas, x: usize, y: usize) {
        const BLACK: u32 = 0x000000ff;
        const GRAY0: u32 = 0x466a92ff;
        const GRAY1: u32 = 0xbaecffff;

        <Self as Drawable>::draw_sprite(canvas, x, y, 11, 13, &[
                0,     0,     0,     0, BLACK, BLACK, BLACK, BLACK,     0,     0,     0,
                0,     0, BLACK, BLACK, GRAY1, GRAY1, GRAY1, GRAY1, BLACK,     0,     0,
                0, BLACK, GRAY1, GRAY1, GRAY1, GRAY1, GRAY1, GRAY1, GRAY1, BLACK,     0,
                0, BLACK, GRAY1, GRAY1, GRAY1, GRAY1, GRAY1, GRAY1, GRAY1, GRAY0, BLACK,
            BLACK, GRAY1, GRAY1, GRAY1, GRAY1, GRAY1, BLACK, BLACK, GRAY1, GRAY0, BLACK,
            BLACK, BLACK, BLACK, GRAY1, GRAY1, BLACK, BLACK, BLACK, BLACK, GRAY0, BLACK,
            BLACK, BLACK, BLACK, GRAY1, GRAY1, BLACK, BLACK, BLACK, BLACK, GRAY0, BLACK,
            BLACK, BLACK, BLACK, GRAY1, GRAY1, BLACK, BLACK, BLACK, BLACK, GRAY0, BLACK,
            BLACK, GRAY0, GRAY0, BLACK, GRAY1, GRAY1, BLACK, BLACK, GRAY0, GRAY0, BLACK,
                0, BLACK, GRAY0, GRAY1, GRAY1, GRAY1, GRAY0, GRAY0, GRAY0, BLACK,     0,
                0,     0, BLACK, GRAY0, BLACK, GRAY0, BLACK, GRAY0, BLACK,     0,     0,
                0,     0, BLACK, GRAY0, BLACK, GRAY0, BLACK, GRAY0, BLACK,     0,     0,
                0,     0,     0, BLACK,     0, BLACK,     0, BLACK,     0,     0,     0,
        ]);
    }
}
