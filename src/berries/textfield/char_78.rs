use super::{
    Canvas,
    Drawable,
};

pub struct Char;

impl Drawable for Char {
    fn draw(&self, canvas: &mut dyn Canvas, x: usize, y: usize) {
        const B: u32 = 0x000000ff;
        const G: u32 = 0x37946eff;
        const W: u32 = 0xffffffff;

        <Self as Drawable>::draw_sprite(canvas, x, y, 5, 6, &[
            0, B, 0, B, 0,
            B, W, B, W, B,
            B, G, W, G, B,
            B, W, G, W, B,
            B, G, B, G, B,
            0, B, 0, B, 0,
        ]);
    }
}
