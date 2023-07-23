use super::{
    Canvas,
    Drawable,
};

pub struct MoonBerry;

impl Drawable for MoonBerry {
    fn draw(&self, canvas: &mut dyn Canvas, x: usize, y: usize) {
        const BLACK: u32 = 0x000000ff;
        const BLUE0: u32 = 0x164574ff;
        const BLUE1: u32 = 0x2677d9ff;
        const GREN0: u32 = 0x36ba83ff;
        const GREN1: u32 = 0x5cff6fff;
        const PURPL: u32 = 0x523493ff;
        const WHITE: u32 = 0xffffffff;
        const YELLO: u32 = 0xe1ff6bff;

        <Self as Drawable>::draw_sprite(canvas, x, y, 16, 13, &[
                0,     0,     0,     0,     0,     0,     0, BLACK, BLACK,     0,     0,     0,     0,     0,     0,     0,
                0,     0,     0,     0,     0,     0, BLACK, BLACK, BLUE1, BLACK,     0,     0,     0,     0,     0,     0,
                0,     0,     0,     0,     0, BLACK, BLUE1, BLACK, PURPL, BLUE1, BLACK,     0,     0,     0,     0,     0,
                0,     0,     0,     0, BLACK, BLUE1, BLUE0, BLUE1, BLUE1, BLUE0, BLACK, BLACK,     0,     0,     0,     0,
                0,     0,     0,     0, BLACK, BLUE0, GREN0, BLUE0, BLUE0, GREN0, BLUE0, BLACK,     0,     0,     0,     0,
                0, YELLO, WHITE, BLACK, GREN0, GREN1, GREN1, GREN1, GREN1, YELLO, GREN0, GREN0, BLACK, WHITE, YELLO,     0,
            YELLO, WHITE,     0, BLACK, YELLO, GREN1, GREN1, YELLO, GREN1, GREN1, GREN1, YELLO, BLACK,     0, WHITE, YELLO,
            YELLO, WHITE,     0, BLACK, GREN0, GREN1, GREN1, GREN0, GREN1, YELLO, GREN1, GREN0, BLACK,     0, WHITE, YELLO,
                0, YELLO, WHITE, BLACK, GREN1, GREN0, GREN1, GREN1, GREN1, GREN0, GREN0, GREN1, BLACK, WHITE, YELLO,     0,
                0,     0,     0, YELLO, WHITE, WHITE, YELLO, GREN0, GREN0, YELLO, WHITE, WHITE, YELLO,     0,     0,     0,
                0,     0,     0,     0,     0, BLACK, WHITE, WHITE, WHITE, WHITE, BLACK,     0,     0,     0,     0,     0,
                0,     0,     0,     0,     0,     0, BLACK, GREN1, GREN1, BLACK,     0,     0,     0,     0,     0,     0,
                0,     0,     0,     0,     0,     0,     0, BLACK, BLACK,     0,     0,     0,     0,     0,     0,     0,
        ]);
    }
}
