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

flag_horiz!(AroaceFlagGen, AroaceFlag, &[
    (0xe28c00ff, 1), (0xeccd00ff, 1), (0xffffffff, 1), (0x62aedcff, 1), (0x203856ff, 1),
]);

flag_horiz!(AromanticFlagGen, AromanticFlag, &[
    (0x3ba740ff, 1), (0xa8d47aff, 1), (0xffffffff, 1), (0xabababff, 1), (0x000000ff, 1),
]);

flag_horiz!(AsexualFlagGen, AsexualFlag, &[
    (0x000000ff, 1), (0xa4a4a4ff, 1), (0xffffffff, 1), (0x810081ff, 1),
]);

flag_horiz!(BisexualFlagGen, BisexualFlag, &[
    (0xd60270ff, 2), (0x9b4f96ff, 1), (0x0038a8ff, 2),
]);

flag_horiz!(EnbyFlagGen, EnbyFlag, &[
    (0xfcf431ff, 1), (0xfcfcfcff, 1), (0x9d59d2ff, 1), (0x282828ff, 1),
]);

flag_horiz!(GayFlagGen, GayFlag, &[
    (0x018e71ff, 1), (0x21cfabff, 1), (0x99e9c2ff, 1), (0xffffffff, 1), (0x7cafe3ff, 1), (0x4f47cdff, 1), (0x3b1379ff, 1),
]);

flag_horiz!(LesbianFlagGen, LesbianFlag, &[
    (0xd62800ff, 1), (0xf07527ff, 1), (0xff9b56ff, 1), (0xffffffff, 1), (0xd462a6ff, 1), (0xb55592ff, 1), (0xa40062ff, 1),
]);

flag_horiz!(Lesbian5FlagGen, Lesbian5Flag, &[
    (0xd62800ff, 1), (0xff9b56ff, 1), (0xffffffff, 1), (0xd462a6ff, 1), (0xa40062ff, 1),
]);

flag_horiz!(QueerFlagGen, QueerFlag, &[
    (0x000000ff, 1), (0x9ad9eaff, 1), (0x00a3e8ff, 1), (0xb5e51dff, 1), (0xffffffff, 1), (0xffc90dff, 1), (0xfc6667ff, 1), (0xfeaec9ff, 1), (0x000000ff, 1),
]);

flag_horiz!(RainbowFlagGen, RainbowFlag, &[
    (0xe50000ff, 1), (0xff8d00ff, 1), (0xffee00ff, 1), (0x028121ff, 1), (0x004cffff, 1), (0x770088ff, 1),
]);

flag_horiz!(TransFlagGen, TransFlag, &[
    (0x5bcefaff, 1), (0xf5a9b8ff, 1), (0xffffffff, 1), (0xf5a9b8ff, 1), (0x5bcefaff, 1),
]);
