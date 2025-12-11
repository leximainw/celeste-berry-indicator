use crate::Image;
use super::Color;

pub trait Metagenerator {
    fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator>;

    fn get_pixel(&self, point: (usize, usize), size: (usize, usize)) -> (Color, u16) {
        self.get_generator(size).get_pixel(point)
    }

    fn draw_under(&self, image: &mut dyn Image) {
        let width = image.get_width();
        let height = image.get_height();
        let generator = self.get_generator((width, height));
        for y in 0..height {
            for x in 0..width {
                let point = (x, y);
                image.set_pixel(point, Color::alpha_over(image.get_pixel(point), generator.get_pixel(point).0));
            }
        }
    }
}

pub trait Generator {
    fn get_pixel(&self, point: (usize, usize)) -> (Color, u16);
}
