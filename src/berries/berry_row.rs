use crate::image::Color;

use super::{
    Canvas,
    Drawable,
};

pub struct BerryRow {
    color: u32,
    berries: Vec<Vec<bool>>,
}

impl BerryRow {
    pub fn new(color: u32) -> BerryRow {
        BerryRow{
            color,
            berries: Vec::new(),
        }
    }

    pub fn from_vec(color: u32, vec: Vec<Vec<bool>>) -> BerryRow {
        BerryRow{
            color,
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
        let desat: Color = Color::from_srgba32(self.color).desaturate();

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
                    canvas.set_pixel(x, y, self.color);
                } else {
                    canvas.set_pixel(x, y, Color::average(desat, Color::from_srgba32(canvas.get_pixel(x, y))).to_srgba32());
                }
                x += 1;
            }
        }
    }
}
