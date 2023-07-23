use crate::image::Color;

use super::{
    Canvas,
    Drawable,
};

pub struct BerryRow {
    berries: Vec<Vec<bool>>,
}

impl BerryRow {
    pub fn new() -> BerryRow {
        BerryRow{
            berries: Vec::new(),
        }
    }

    pub fn from_vec(vec: Vec<Vec<bool>>) -> BerryRow {
        BerryRow{
            berries: vec,
        }
    }

    pub fn push(&mut self, vec: Vec<bool>) {
        self.berries.push(vec);
    }
}

impl Drawable for BerryRow {
    fn draw(&self, canvas: &mut dyn Canvas, mut x: usize, y: usize) {
        const BLACK: u32 = 0x000000ff;
        const RED: u32 = 0xde2a2aff;
        let desat_red: Color = Color::from_srgba32(RED).desaturate();

        for i in 0..self.berries.len() {
            if i != 0 {
                for j in 0..3 {
                    canvas.set_pixel(x + 1, y + j - 1, BLACK);
                }
                x += 3;
            }
            let level = &self.berries[i];
            for j in 0..level.len() {
                if level[j] {
                    canvas.set_pixel(x, y, RED);
                } else {
                    canvas.set_pixel(x, y, Color::average(desat_red, Color::from_srgba32(canvas.get_pixel(x, y))).to_srgba32());
                }
                x += 1;
            }
        }
    }
}
