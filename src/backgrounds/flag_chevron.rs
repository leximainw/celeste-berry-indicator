use super::{Color, Generator, Metagenerator};

pub struct FlagChevronGen {
    chevron: &'static [u32],
    width_permille: usize,
    intercept: usize,
}

impl FlagChevronGen {
    pub fn new(chevron: &'static [u32], width_permille: usize, intercept: usize) -> Self {
        Self{chevron, width_permille, intercept}
    }
}

impl Metagenerator for FlagChevronGen {
    fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator> {
        let chevron_width = usize::min(size.0 * self.width_permille / 1000, size.1 * self.width_permille / 1000);
        Box::new(FlagChevron{
            chevron: self.chevron,
            chevron_delta: (size.1 / 2 + self.intercept * chevron_width) as isize,
            chevron_width: chevron_width,
            half_height: size.1 as isize / 2,
        })
    }
}

pub struct FlagChevron {
    chevron: &'static [u32],
    chevron_delta: isize,
    chevron_width: usize,
    half_height: isize,
}

impl Generator for FlagChevron {
    fn get_pixel(&self, pixel: (usize, usize)) -> (Color, u16) {
        let offset = -(pixel.1 as isize - self.half_height).wrapping_abs();
        let effective_x = offset + self.chevron_delta - pixel.0 as isize;
        if effective_x < 0 {
            (Color::ZERO, 0)
        } else if effective_x as usize > (self.chevron.len() - 1) * self.chevron_width {
            (Color::from_srgba32(self.chevron[self.chevron.len() - 1]), u16::MAX)
        } else {
            let mut effective_x = effective_x as usize;
            for (index, stripe) in self.chevron.iter().enumerate() {
                if effective_x == 0 {
                    if index == 0 {
                        return (Color::from_srgba32(self.chevron[0]), u16::MAX / 2);
                    } else {
                        return (Color::lerp(
                            Color::from_srgba32(self.chevron[index - 1]),
                            Color::from_srgba32(self.chevron[index]),
                            (1, 2.try_into().unwrap())
                        ), u16::MAX);
                    }
                } else if effective_x < self.chevron_width {
                    return (Color::from_srgba32(self.chevron[index]), u16::MAX);
                } else {
                    effective_x -= self.chevron_width;
                }
            }
            unsafe{ std::hint::unreachable_unchecked() };
        }
    }
}
