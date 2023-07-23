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

        <Self as Drawable>::draw_sprite(canvas, x, y, 5, 8, &[
            0, B, B, B, 0,
            B, W, W, W, B,
            B, G, G, W, B,
            B, W, W, W, B,
            B, G, G, W, B,
            B, W, W, W, B,
            B, G, G, G, B,
            0, B, B, B, 0,
        ]);
    }
}
