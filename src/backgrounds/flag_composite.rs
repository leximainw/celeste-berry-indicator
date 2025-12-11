use super::{Color, Generator, Metagenerator};

pub struct FlagCompositeGen {
    metagenerators: &'static [&'static dyn Metagenerator],
}

impl FlagCompositeGen {
    pub fn new(metagenerators: &'static [&'static dyn Metagenerator]) -> Self {
        Self{metagenerators}
    }
}

impl Metagenerator for FlagCompositeGen {
    fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator> {
        Box::new(FlagComposite{
            generators: self.metagenerators.iter().map(|x| x.get_generator(size)).collect(),
        })
    }
}

pub struct FlagComposite {
    generators: Vec<Box<dyn Generator>>,
}

impl Generator for FlagComposite {
    fn get_pixel(&self, pixel: (usize, usize)) -> (Color, u16) {
        let mut colors = vec![(Color::ZERO, 0); self.generators.len()];
        for (index, generator) in self.generators.iter().enumerate().rev() {
            let color = generator.get_pixel(pixel);
            colors[index] = color;
            if color.1 == u16::MAX {
                return colors.into_iter().skip(index).reduce(|l, r| {
                    (Color::lerp(l.0, r.0, (r.1.into(), std::num::NonZeroU32::new(65535).unwrap())), u16::MAX)
                }).unwrap_or((Color::ZERO, 0));
            }
        }
        colors.into_iter().reduce(|l, r| {
            (Color::lerp(l.0, r.0, (r.1.into(), std::num::NonZeroU32::new(65535).unwrap())), u16::MAX)
        }).unwrap_or((Color::ZERO, 0))
    }
}
