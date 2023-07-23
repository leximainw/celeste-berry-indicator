use super::{
    Canvas,
    Drawable,
};

pub struct WingedGoldenBerry;

impl Drawable for WingedGoldenBerry {
    fn draw(canvas: &mut dyn Canvas, x: usize, y: usize) {
        const BLK: u32 = 0x000000ff;
        const GRY: u32 = 0xcbdbfcff;
        const RED: u32 = 0xbc1616ff;
        const WHT: u32 = 0xffffffff;
        const YL0: u32 = 0x755716ff;
        const YL1: u32 = 0x8a760fff;
        const YL2: u32 = 0xdec02aff;
        const YL3: u32 = 0xffee42ff;

        <Self as Drawable>::draw_sprite(canvas, x, y, 34, 14, &[
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, BLK, BLK,   0,   0,   0, BLK, BLK,   0,   0,   0, BLK, BLK,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, BLK, YL1, BLK,   0, BLK, YL2, YL2, BLK,   0, BLK, YL1, BLK,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, BLK, YL1, YL2, BLK, YL2, RED, RED, YL2, BLK, YL2, YL1, BLK,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, BLK, YL0, YL2, YL2, YL2, RED, RED, YL2, YL2, YL2, YL0, BLK,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0, BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK,   0, BLK, YL0, YL1, YL2, YL2, YL1, YL1, YL2, YL2, YL1, YL0, BLK,   0, BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK,   0,
            BLK,WHT,WHT, WHT, WHT, WHT, WHT, WHT, WHT, WHT, BLK,   0, BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK,   0, BLK, WHT, WHT, WHT, WHT, WHT, WHT, WHT, WHT, WHT, BLK,
            0, BLK, GRY, GRY, GRY, GRY, GRY, GRY, WHT, WHT, WHT, BLK, BLK, YL1, YL3, YL2, YL2, YL2, YL3, YL2, YL1, BLK, BLK, WHT, WHT, WHT, GRY, GRY, GRY, GRY, GRY, GRY, BLK,   0,
            0,   0, BLK, BLK, WHT, WHT, GRY, WHT, WHT, GRY, WHT, GRY, BLK, YL2, YL1, YL2, YL3, YL2, YL1, YL2, YL2, BLK, GRY, WHT, GRY, WHT, WHT, GRY, WHT, WHT, BLK, BLK,   0,   0,
            0,   0,   0,   0, BLK, WHT, GRY, GRY, GRY, GRY, WHT, GRY, BLK, YL2, YL2, YL2, YL1, YL2, YL2, YL3, YL2, BLK, GRY, WHT, GRY, GRY, GRY, GRY, WHT, BLK,   0,   0,   0,   0,
            0,   0,   0,   0,   0, BLK, BLK, WHT, WHT, GRY, WHT, BLK, BLK, YL2, YL1, YL2, YL2, YL2, YL2, YL1, YL2, BLK, BLK, WHT, GRY, WHT, WHT, BLK, BLK,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0, BLK, BLK, WHT, GRY, BLK,   0, BLK, YL2, YL1, YL1, YL1, YL1, YL2, BLK,   0, BLK, GRY, WHT, BLK, BLK,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0, BLK, BLK,   0,   0,   0, BLK, YL2, YL1, YL1, YL2, BLK,   0,   0,   0, BLK, BLK,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, BLK, YL2, YL2, BLK,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, BLK, BLK,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
        ]);
    }
}
