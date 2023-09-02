use super::{
    Color,
    FlagStripes,
    Generator,
    Metagenerator,
};

pub struct FlagGenerator;

impl Metagenerator for FlagGenerator {
    fn get_generator(size: (usize, usize)) -> Box<dyn Generator> {
        Box::new(Flag{
            width: size.0,
            height: size.1,
        })
    }
}

pub struct Flag {
    width: usize,
    height: usize,
}

impl Generator for Flag {
    fn get_pixel(&self, pixel: (usize, usize)) -> Color {
        const STRIPES: &'static [(u32, usize)] = &[
            (0x5bcefaff, 1), (0xf5a9b8ff, 1), (0xffffffff, 1), (0xf5a9b8ff, 1), (0x5bcefaff, 1)
        ];
        FlagStripes::stripes(STRIPES, pixel.1, self.height)
    }
}
