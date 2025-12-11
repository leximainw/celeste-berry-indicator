use super::{Color, Generator, Metagenerator};

pub struct FlagStripeGen {
    stripes: &'static [u32],
}

impl FlagStripeGen {
    pub fn new(stripes: &'static [u32]) -> Self {
        Self{stripes}
    }
}

impl Metagenerator for FlagStripeGen {
    fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator> {
        Box::new(FlagStripe{
            stripes: self.stripes,
            size: size.1,
        })
    }
}

pub struct FlagStripe {
    stripes: &'static [u32],
    size: usize,
}

impl Generator for FlagStripe {
    fn get_pixel(&self, pixel: (usize, usize)) -> (Color, u16) {
        let weight: usize = self.stripes.len();
        let lower_block = pixel.1 * weight / self.size;
        let upper_block = (pixel.1 + 1) * weight / self.size;
        if lower_block == upper_block || lower_block + 1 == weight {
            (Color::from_srgba32(self.stripes[lower_block]), u16::MAX)
        } else if lower_block + 1 == upper_block {
            let lower_weight = pixel.1 * self.size % weight;
            (Color::lerp(Color::from_srgba32(self.stripes[lower_block]),
                Color::from_srgba32(self.stripes[upper_block]),
                (lower_weight as u32, std::num::NonZeroU32::new(weight as u32).unwrap())), u16::MAX)
        } else {
            todo!();
        }
    }
}
