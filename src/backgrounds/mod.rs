use crate::image::Color as Color;

mod generator;
pub use generator::Metagenerator as Metagenerator;
pub use generator::Generator as Generator;

mod flag_chevron;
pub use flag_chevron::FlagChevronGen as FlagChevronGen;
pub use flag_chevron::FlagChevron as FlagChevron;

mod flag_composite;
pub use flag_composite::FlagCompositeGen as FlagCompositeGen;
pub use flag_composite::FlagComposite as FlagComposite;

mod flag_stripes;
pub use flag_stripes::FlagStripeGen as FlagStripeGen;
pub use flag_stripes::FlagStripe as FlagStripe;

macro_rules! flag_chevron {
    ($vis:vis $gen:ident, $chevron:expr, $width_permille:expr, $intercept:expr) => {
        $vis struct $gen;

        impl Metagenerator for $gen {
            fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator> {
                FlagChevronGen::new($chevron, $width_permille, $intercept).get_generator(size)
            }
        }
    };
}

macro_rules! flag_composite {
    ($vis:vis $gen:ident, $metagenerators:expr) => {
        $vis struct $gen;

        impl Metagenerator for $gen {
            fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator> {
                FlagCompositeGen::new($metagenerators).get_generator(size)
            }
        }
    };
}

macro_rules! flag_stripes {
    ($vis:vis $gen:ident, $stripes:expr) => {
        $vis struct $gen;

        impl Metagenerator for $gen {
            fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator> {
                FlagStripeGen::new($stripes).get_generator(size)
            }
        }
    };
}

flag_stripes!(pub AroaceFlagGen, &[0xe28c00ff, 0xeccd00ff, 0xffffffff, 0x62aedcff, 0x203856ff]);
flag_stripes!(pub AromanticFlagGen, &[0x3ba740ff, 0xa8d47aff, 0xffffffff, 0xabababff, 0x000000ff]);
flag_stripes!(pub AsexualFlagGen, &[0x000000ff, 0xa4a4a4ff, 0xffffffff, 0x810081ff]);
flag_stripes!(pub BisexualFlagGen, &[0xd60270ff, 0xd60270ff, 0x9b4f96ff, 0x0038a8ff, 0x0038a8ff]);
flag_stripes!(pub EnbyFlagGen, &[0xfcf431ff, 0xfcfcfcff, 0x9d59d2ff, 0x282828ff]);
flag_stripes!(pub GayFlagGen, &[0x018e71ff, 0x21cfabff, 0x99e9c2ff, 0xffffffff, 0x7cafe3ff, 0x4f47cdff, 0x3b1379ff]);
flag_stripes!(pub LesbianFlagGen, &[0xd62800ff, 0xf07527ff, 0xff9b56ff, 0xffffffff, 0xd462a6ff, 0xb55592ff, 0xa40062ff]);
flag_stripes!(pub Lesbian5FlagGen, &[0xd62800ff, 0xff9b56ff, 0xffffffff, 0xd462a6ff, 0xa40062ff]);
flag_stripes!(pub QueerFlagGen, &[0x000000ff, 0x9ad9eaff, 0x00a3e8ff, 0xb5e51dff, 0xffffffff, 0xffc90dff, 0xfc6667ff, 0xfeaec9ff, 0x000000ff]);
flag_stripes!(pub RainbowFlagGen, &[0xe50000ff, 0xff8d00ff, 0xffee00ff, 0x028121ff, 0x004cffff, 0x770088ff]);
flag_stripes!(pub TransFlagGen, &[0x5bcefaff, 0xf5a9b8ff, 0xffffffff, 0xf5a9b8ff, 0x5bcefaff]);
flag_composite!(pub ProgressPrideFlagGen, &[&RainbowFlagGen, &ProgressChevronGen]);
flag_composite!(pub IntersexProgressFlagGen, &[&RainbowFlagGen, &IntersexChevronGen, &IntersexCircleGen]);

flag_chevron!(ProgressChevronGen, &[0x000000ff, 0x613915ff, 0x5bcefaff, 0xf5a9b8ff, 0xffffffff], 100, 2);
flag_chevron!(IntersexChevronGen, &[0x000000ff, 0x613915ff, 0x5bcefaff, 0xf5a9b8ff, 0xffffffff, 0xffd800ff], 100, 4);

struct IntersexCircleGen;

impl Metagenerator for IntersexCircleGen {
    fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator> {
        let chevron_width = usize::min(size.0, size.1) / 10;
        let outer_radius = (size.1 - chevron_width * 11 / 3) * 204 / 985;   // x * 204 / 985 ~= x * (sqrt(2) - 1) / 2
        let inner_radius = outer_radius * 2 / 3;
        Box::new(IntersexFlag{
            center_x: chevron_width / 3 + outer_radius,
            center_y: size.1 / 2,
            outer_radsqr: outer_radius * outer_radius,
            inner_radsqr: inner_radius * inner_radius,
            transparent: true,
        })
    }
}

pub struct IntersexFlagGen;

impl Metagenerator for IntersexFlagGen {
    fn get_generator(&self, size: (usize, usize)) -> Box<dyn Generator> {
        let outer_radius = usize::min(size.0, size.1) * 3 / 10;
        let inner_radius = usize::min(size.0, size.1) * 1 / 5;
        Box::new(IntersexFlag{
            center_x: size.0 / 2,
            center_y: size.1 / 2,
            outer_radsqr: outer_radius * outer_radius,
            inner_radsqr: inner_radius * inner_radius,
            transparent: false,
        })
    }
}

pub struct IntersexFlag {
    center_x: usize,
    center_y: usize,
    outer_radsqr: usize,
    inner_radsqr: usize,
    transparent: bool,
}

impl Generator for IntersexFlag {
    fn get_pixel(&self, pixel: (usize, usize)) -> (Color, u16) {
        let dx = if self.center_x < pixel.0 { pixel.0 - self.center_x } else { self.center_x - pixel.0 };
        let dy = if self.center_y < pixel.1 { pixel.1 - self.center_y } else { self.center_y - pixel.1 };
        let r = dx * dx + dy * dy;
        let dr = (dx + 1) * (dx + 1) + (dy + 1) * (dy + 1) - r;
        if r + dr < self.outer_radsqr && r > self.inner_radsqr + dr {
            (Color::from_srgba32(0x7902aaff), u16::MAX)
        } else if r > self.outer_radsqr + dr || r + dr < self.inner_radsqr {
            if self.transparent {
                (Color::ZERO, 0)
            } else {
                (Color::from_srgba32(0xffd800ff), u16::MAX)
            }
        } else {
            let mut frag_count = 0;
            let center_x = self.center_x * 32 + 15;
            let center_y = self.center_y * 32 + 15;
            let orr = self.outer_radsqr * 1024;
            let irr = self.inner_radsqr * 1024;
            for fx in 0..16 {
                for fy in 0..16 {
                    let px = pixel.0 * 32 + fx * 2;
                    let py = pixel.1 * 32 + fy * 2;
                    let dx = if center_x < px { px - center_x } else { center_x - px };
                    let dy = if center_y < py { py - center_y } else { center_y - py };
                    let r = dx * dx + dy * dy;
                    if r >= irr && r <= orr {
                        frag_count += 1;
                    }
                }
            }
            if self.transparent {
                (Color::from_srgba32(0x7902aaff), (frag_count as u16).saturating_mul(256))
            } else {
                (Color::lerp(Color::from_srgba32(0xffd800ff), Color::from_srgba32(0x7902aaff), (frag_count, 256.try_into().unwrap())), u16::MAX)
            }
        }
    }
}
