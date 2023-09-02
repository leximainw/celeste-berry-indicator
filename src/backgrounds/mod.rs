use crate::image::Color as Color;

mod generator;
pub use generator::Metagenerator as Metagenerator;
pub use generator::Generator as Generator;

mod flag_stripes;
pub use flag_stripes::FlagStripes as FlagStripes;

macro_rules! flag_horiz {
    ($gen:ident, $flag:ident, $stripes:expr) => {
        pub struct $gen;

        impl Metagenerator for $gen {
            fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator> {
                Box::new($flag{
                    width: size.0,
                    height: size.1,
                })
            }
        }

        pub struct $flag {
            width: usize,
            height: usize,
        }

        impl Generator for $flag {
            fn get_pixel(&self, pixel: (usize, usize)) -> Color {
                const STRIPES: &'static [(u32, usize)] = $stripes;
                FlagStripes::stripes(STRIPES, pixel.1, self.height)
            }
        }
    };
}

flag_horiz!(EnbyFlagGen, EnbyFlag, &[
    (0xfcf431ff, 1), (0xfcfcfcff, 1), (0x9d59d2ff, 1), (0x282828ff, 1),
]);

flag_horiz!(TransFlagGen, TransFlag, &[
    (0x5bcefaff, 1), (0xf5a9b8ff, 1), (0xffffffff, 1), (0xf5a9b8ff, 1), (0x5bcefaff, 1),
]);
