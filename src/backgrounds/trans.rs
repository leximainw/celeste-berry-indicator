use super::{
    Color,
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
        let weight: usize = STRIPES.iter().map(|x| x.1).sum();
        let pos = pixel.1 % self.height;
        let lower_block = pos * weight / self.height;
        let upper_block = (pos + 1) * weight / self.height;
        if lower_block == upper_block || lower_block + 1 == weight {
            Color::from_srgba32(STRIPES[lower_block].0)
        } else if lower_block + 1 == upper_block {
            let lower_weight = pos * self.height % weight;
            Color::lerp(Color::from_srgba32(STRIPES[lower_block].0),
                Color::from_srgba32(STRIPES[upper_block].0),
                (lower_weight as u32, std::num::NonZeroU32::new(weight as u32).unwrap()))
        } else {
            todo!();
        }
    }
}
