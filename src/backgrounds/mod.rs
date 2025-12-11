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
                Box::new($flag(size.1))
            }
        }

        pub struct $flag(usize);

        impl Generator for $flag {
            fn get_pixel(&self, pixel: (usize, usize)) -> Color {
                const STRIPES: &'static [u32] = $stripes;
                FlagStripes::stripes(STRIPES, pixel.1, self.0)
            }
        }
    };
}

flag_horiz!(AroaceFlagGen, AroaceFlag, &[
    0xe28c00ff, 0xeccd00ff, 0xffffffff, 0x62aedcff, 0x203856ff,
]);

flag_horiz!(AromanticFlagGen, AromanticFlag, &[
    0x3ba740ff, 0xa8d47aff, 0xffffffff, 0xabababff, 0x000000ff,
]);

flag_horiz!(AsexualFlagGen, AsexualFlag, &[
    0x000000ff, 0xa4a4a4ff, 0xffffffff, 0x810081ff,
]);

flag_horiz!(BisexualFlagGen, BisexualFlag, &[
    0xd60270ff, 0xd60270ff, 0x9b4f96ff, 0x0038a8ff, 0x0038a8ff,
]);

flag_horiz!(EnbyFlagGen, EnbyFlag, &[
    0xfcf431ff, 0xfcfcfcff, 0x9d59d2ff, 0x282828ff,
]);

flag_horiz!(GayFlagGen, GayFlag, &[
    0x018e71ff, 0x21cfabff, 0x99e9c2ff, 0xffffffff, 0x7cafe3ff, 0x4f47cdff, 0x3b1379ff,
]);

flag_horiz!(LesbianFlagGen, LesbianFlag, &[
    0xd62800ff, 0xf07527ff, 0xff9b56ff, 0xffffffff, 0xd462a6ff, 0xb55592ff, 0xa40062ff,
]);

flag_horiz!(Lesbian5FlagGen, Lesbian5Flag, &[
    0xd62800ff, 0xff9b56ff, 0xffffffff, 0xd462a6ff, 0xa40062ff,
]);

flag_horiz!(QueerFlagGen, QueerFlag, &[
    0x000000ff, 0x9ad9eaff, 0x00a3e8ff, 0xb5e51dff, 0xffffffff, 0xffc90dff, 0xfc6667ff, 0xfeaec9ff, 0x000000ff,
]);

flag_horiz!(RainbowFlagGen, RainbowFlag, &[
    0xe50000ff, 0xff8d00ff, 0xffee00ff, 0x028121ff, 0x004cffff, 0x770088ff,
]);

flag_horiz!(TransFlagGen, TransFlag, &[
    0x5bcefaff, 0xf5a9b8ff, 0xffffffff, 0xf5a9b8ff, 0x5bcefaff,
]);
