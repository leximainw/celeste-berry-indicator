#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u32,
    g: u32,
    b: u32,
    a: u32,
}

impl Color {
    pub const ZERO: Color = Color{
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    const GAMMA_TABLE: [u32; 256] = [
        0x00000000, 0x00005096, 0x000174db, 0x0003917e, 0x0006bd1e, 0x000b08d0, 0x0010827f, 0x00173600,
        0x001f2db1, 0x002872cf, 0x00330db7, 0x003f0612, 0x004c62f3, 0x005b2aee, 0x006b642c, 0x007d147a,
        0x00904151, 0x00a4efe3, 0x00bb2523, 0x00d2e5c8, 0x00ec3656, 0x01071b24, 0x0123985a, 0x0141b1fc,
        0x01616be8, 0x0182c9db, 0x01a5cf73, 0x01ca8031, 0x01f0df7c, 0x0218f0a0, 0x0242b6d3, 0x026e3535,
        0x029b6ed0, 0x02ca669c, 0x02fb1f7c, 0x032d9c45, 0x0361dfb9, 0x0397ec8a, 0x03cfc55c, 0x04096cc5,
        0x0444e54c, 0x0482316c, 0x04c15393, 0x05024e25, 0x05452377, 0x0589d5d7, 0x05d06786, 0x0618dabb,
        0x066331a5, 0x06af6e69, 0x06fd9322, 0x074da1e4, 0x079f9cb8, 0x07f385a3, 0x08495e9e, 0x08a1299e,
        0x08fae88e, 0x09569d55, 0x09b449cf, 0x0a13efd6, 0x0a75913a, 0x0ad92fc8, 0x0b3ecd43, 0x0ba66b6b,
        0x0c100bfb, 0x0c7bb0a7, 0x0ce95b1d, 0x0d590d07, 0x0dcac809, 0x0e3e8dc4, 0x0eb45fd2, 0x0f2c3fca,
        0x0fa62f3d, 0x10222fb8, 0x10a042c5, 0x112069e7, 0x11a2a69f, 0x1226fa6b, 0x12ad66c2, 0x1335ed19,
        0x13c08ee1, 0x144d4d87, 0x14dc2a75, 0x156d2712, 0x160044c0, 0x169584de, 0x172ce8c8, 0x17c671d7,
        0x18622162, 0x18fff8ba, 0x199ff930, 0x1a42240f, 0x1ae67aa3, 0x1b8cfe30, 0x1c35affc, 0x1ce09149,
        0x1d8da353, 0x1e3ce759, 0x1eee5e92, 0x1fa20a36, 0x2057eb7a, 0x2110038f, 0x21ca53a4, 0x2286dce8,
        0x2345a084, 0x24069fa2, 0x24c9db67, 0x258f54f8, 0x26570d77, 0x27210603, 0x27ed3fbb, 0x28bbbbb9,
        0x298c7b18, 0x2a5f7eef, 0x2b34c853, 0x2c0c5859, 0x2ce63012, 0x2dc2508f, 0x2ea0badd, 0x2f817008,
        0x3064711b, 0x3149bf1f, 0x32315b1b, 0x331b4614, 0x3407810d, 0x34f60d09, 0x35e6eb07, 0x36da1c08,
        0x37cfa106, 0x38c77aff, 0x39c1aaec, 0x3abe31c5, 0x3bbd1082, 0x3cbe4817, 0x3dc1d97a, 0x3ec7c59c,
        0x3fd00d6e, 0x40dab1e0, 0x41e7b3e1, 0x42f7145d, 0x4408d440, 0x451cf475, 0x463375e4, 0x474c5974,
        0x4867a00d, 0x49854a94, 0x4aa559eb, 0x4bc7cef6, 0x4cecaa96, 0x4e13edab, 0x4f3d9915, 0x5069adb0,
        0x51982c5b, 0x52c915ef, 0x53fc6b48, 0x55322d3e, 0x566a5caa, 0x57a4fa63, 0x58e2073f, 0x5a218412,
        0x5b6371b0, 0x5ca7d0ed, 0x5deea299, 0x5f37e785, 0x6083a082, 0x61d1ce5d, 0x632271e4, 0x64758be4,
        0x65cb1d2a, 0x6723267e, 0x687da8ac, 0x69daa47c, 0x6b3a1ab5, 0x6c9c0c20, 0x6e007982, 0x6f6763a0,
        0x70d0cb3e, 0x723cb121, 0x73ab160a, 0x751bfabb, 0x768f5ff4, 0x78054677, 0x797daf02, 0x7af89a53,
        0x7c760927, 0x7df5fc3c, 0x7f78744d, 0x80fd7215, 0x8284f64f, 0x840f01b4, 0x859b94fc, 0x872ab0df,
        0x88bc5616, 0x8a508555, 0x8be73f53, 0x8d8084c4, 0x8f1c565d, 0x90bab4d1, 0x925ba0d3, 0x93ff1b15,
        0x95a52448, 0x974dbd1d, 0x98f8e643, 0x9aa6a06a, 0x9c56ec40, 0x9e09ca74, 0x9fbf3bb1, 0xa17740a6,
        0xa331d9fd, 0xa4ef0862, 0xa6aecc7f, 0xa87126ff, 0xaa36188a, 0xabfda1c9, 0xadc7c364, 0xaf947e03,
        0xb163d24c, 0xb335c0e5, 0xb50a4a74, 0xb6e16f9e, 0xb8bb3108, 0xba978f54, 0xbc768b26, 0xbe582521,
        0xc03c5de6, 0xc2233618, 0xc40cae57, 0xc5f8c742, 0xc7e7817b, 0xc9d8dda0, 0xcbccdc50, 0xcdc37e28,
        0xcfbcc3c7, 0xd1b8adca, 0xd3b73ccc, 0xd5b8716a, 0xd7bc4c40, 0xd9c2cde7, 0xdbcbf6fb, 0xddd7c815,
        0xdfe641d0, 0xe1f764c2, 0xe40b3186, 0xe621a8b2, 0xe83acade, 0xea5698a2, 0xec751292, 0xee963946,
        0xf0ba0d53, 0xf2e08f4d, 0xf509bfc9, 0xf7359f5b, 0xf9642e97, 0xfb956e0f, 0xfdc95e56, 0xffffffff,
    ];

    const LOOKUP_TABLE: [u32; 256] = [
        0x0000116b, 0x0000c570, 0x0002628a, 0x00050445, 0x0008be07, 0x000d9f20, 0x0013b458, 0x001b08b9,
        0x0023a608, 0x002d950b, 0x0038ddc2, 0x00458785, 0x00539928, 0x00631906, 0x00740d18, 0x00867aff,
        0x009a6812, 0x00afd960, 0x00c6d3bf, 0x00df5bcb, 0x00f975f1, 0x01152671, 0x0132715f, 0x01515aac,
        0x0171e626, 0x0194177a, 0x01b7f236, 0x01dd79cf, 0x0204b19f, 0x022d9ce5, 0x02583ecd, 0x02849a6b,
        0x02b2b2c1, 0x02e28abc, 0x03142537, 0x034784fe, 0x037caccb, 0x03b39f4a, 0x03ec5f15, 0x0426eebd,
        0x046350c1, 0x04a18798, 0x04e195a9, 0x05237d51, 0x056740e1, 0x05ace2a1, 0x05f464cc, 0x063dc997,
        0x0689132a, 0x06d643a6, 0x07255d21, 0x077661ab, 0x07c9534b, 0x081e33ff, 0x087505bf, 0x08cdca7a,
        0x09288419, 0x0985347e, 0x09e3dd84, 0x0a448100, 0x0aa720bf, 0x0b0bbe8b, 0x0b725c26, 0x0bdafb4b,
        0x0c459db3, 0x0cb2450e, 0x0d20f309, 0x0d91a94b, 0x0e046976, 0x0e793527, 0x0ef00df8, 0x0f68f57b,
        0x0fe3ed41, 0x1060f6d4, 0x10e013bb, 0x11614578, 0x11e48d8b, 0x1269ed6d, 0x12f16696, 0x137afa78,
        0x1406aa81, 0x1494781f, 0x152464b8, 0x15b671b1, 0x164aa06b, 0x16e0f243, 0x17796896, 0x181404b8,
        0x18b0c800, 0x194fb3bd, 0x19f0c93e, 0x1a9409ce, 0x1b3976b6, 0x1be1113a, 0x1c8ada9f, 0x1d36d422,
        0x1de4ff03, 0x1e955c7b, 0x1f47edc4, 0x1ffcb411, 0x20b3b097, 0x216ce487, 0x2228510e, 0x22e5f759,
        0x23a5d891, 0x2467f5dd, 0x252c5064, 0x25f2e948, 0x26bbc1aa, 0x2786daa8, 0x28543560, 0x2923d2eb,
        0x29f5b463, 0x2ac9dade, 0x2ba04771, 0x2c78fb2e, 0x2d53f727, 0x2e313c6b, 0x2f10cc06, 0x2ff2a704,
        0x30d6ce6e, 0x31bd434e, 0x32a606a7, 0x33911980, 0x347e7cda, 0x356e31b8, 0x36603918, 0x375493f7,
        0x384b4354, 0x39444828, 0x3a3fa36c, 0x3b3d5618, 0x3c3d6122, 0x3d3fc580, 0x3e448424, 0x3f4b9e00,
        0x40551404, 0x4160e720, 0x426f1841, 0x437fa853, 0x44929842, 0x45a7e8f7, 0x46bf9b59, 0x47d9b052,
        0x48f628c4, 0x4a150597, 0x4b3647ac, 0x4c59efe5, 0x4d7fff24, 0x4ea87648, 0x4fd3562f, 0x51009fb6,
        0x523053ba, 0x53627315, 0x5496fea2, 0x55cdf738, 0x57075db0, 0x584332df, 0x5981779c, 0x5ac22cba,
        0x5c05530e, 0x5d4aeb68, 0x5e92f69a, 0x5fdd7574, 0x612a68c6, 0x6279d15e, 0x63cbb008, 0x65200592,
        0x6676d2c5, 0x67d0186d, 0x692bd753, 0x6a8a103f, 0x6beac3f8, 0x6d4df346, 0x6eb39eed, 0x701bc7b3,
        0x71866e5b, 0x72f393a8, 0x7463385d, 0x75d55d3a, 0x774a0301, 0x78c12a6f, 0x7a3ad445, 0x7bb70141,
        0x7d35b21e, 0x7eb6e799, 0x803aa26f, 0x81c0e358, 0x8349ab10, 0x84d4fa50, 0x8662d1cf, 0x87f33245,
        0x89861c69, 0x8b1b90f1, 0x8cb39092, 0x8e4e1c00, 0x8feb33f1, 0x918ad915, 0x932d0c21, 0x94d1cdc5,
        0x96791eb3, 0x9822ff9b, 0x99cf712b, 0x9b7e7414, 0x9d300903, 0x9ee430a6, 0xa09aebaa, 0xa2543aba,
        0xa4101e83, 0xa5ce97af, 0xa78fa6e8, 0xa9534cd8, 0xab198a28, 0xace25f81, 0xaeadcd89, 0xb07bd4e8,
        0xb24c7644, 0xb41fb244, 0xb5f5898c, 0xb7cdfcc1, 0xb9a90c87, 0xbb86b982, 0xbd670454, 0xbf49eda0,
        0xc12f7608, 0xc3179e2c, 0xc50266ad, 0xc6efd02b, 0xc8dfdb46, 0xcad2889d, 0xccc7d8cd, 0xcebfcc75,
        0xd0ba6432, 0xd2b7a0a1, 0xd4b7825e, 0xd6ba0a04, 0xd8bf382f, 0xdac70d7a, 0xdcd18a7e, 0xdedeafd5,
        0xe0ee7e18, 0xe300f5e0, 0xe51617c5, 0xe72de45f, 0xe9485c44, 0xeb65800b, 0xed85504b, 0xefa7cd98,
        0xf1ccf889, 0xf3f4d1b2, 0xf61f59a6, 0xf84c90fb, 0xfa7c7842, 0xfcaf1010, 0xfee458f5, 0xffffffff,
    ];

    pub fn get_red(&self) -> u32 {
        self.r
    }

    pub fn get_green(&self) -> u32 {
        self.g
    }

    pub fn get_blue(&self) -> u32 {
        self.b
    }

    pub fn get_alpha(&self) -> u32 {
        self.a
    }

    pub fn set_red(&self, value: u32) -> Color {
        Color{
            r: value,
            ..*self
        }
    }

    pub fn set_green(&self, value: u32) -> Color {
        Color{
            g: value,
            ..*self
        }
    }

    pub fn set_blue(&self, value: u32) -> Color {
        Color{
            b: value,
            ..*self
        }
    }

    pub fn set_alpha(&self, value: u32) -> Color {
        Color{
            a: value,
            ..*self
        }
    }

    pub fn from_srgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color{
            r: Self::GAMMA_TABLE[r as usize],
            g: Self::GAMMA_TABLE[g as usize],
            b: Self::GAMMA_TABLE[b as usize],
            a: a as u32 * 0x01010101,
        }
    }

    pub fn from_srgba32(color: u32) -> Color {
        Color{
            r: Self::GAMMA_TABLE[(color >> 24 & 255) as usize],
            g: Self::GAMMA_TABLE[(color >> 16 & 255) as usize],
            b: Self::GAMMA_TABLE[(color >> 8 & 255) as usize],
            a: (color & 255) * 0x01010101,
        }
    }

    pub fn to_srgba32(&self) -> u32 {
        (Self::lookup_srgb(self.r) as u32) << 24 | (Self::lookup_srgb(self.g) as u32) << 16 | (Self::lookup_srgb(self.b) as u32) << 8 | self.a >> 24
    }

    fn lookup_srgb(value: u32) -> u8 {
        let mut curr = 128_u8;
        let mut delta = 128_u8;
        while delta != 0 {
            delta /= 2;
            if value <= Self::LOOKUP_TABLE[curr as usize] {
                if value > Self::LOOKUP_TABLE[curr as usize - 1] {
                    return curr;
                }
                curr -= delta;
            } else {
                curr += delta;
            }
        }
        curr - 1
    }

    pub fn from_rgba_linear(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color{
            r: r as u32 * 0x01010101,
            g: g as u32 * 0x01010101,
            b: b as u32 * 0x01010101,
            a: a as u32 * 0x01010101,
        }
    }

    pub fn from_rgba32_linear(color: u32) -> Color {
        Color{
            r: (color >> 24 & 255) * 0x01010101,
            g: (color >> 16 & 255) * 0x01010101,
            b: (color >> 8 & 255) * 0x01010101,
            a: (color & 255) * 0x01010101,
        }
    }

    pub fn to_rgba32_linear(&self) -> u32 {
        self.r >> 24 << 24 | self.g >> 24 << 16 | self.b >> 24 << 8 | self.a >> 24
    }

    pub fn average(left: Color, right: Color) -> Color {
        Color{
            r: Self::average_channel(left.r, right.r),
            g: Self::average_channel(left.g, right.g),
            b: Self::average_channel(left.b, right.b),
            a: Self::average_channel(left.a, right.a),
        }
    }

    pub fn lerp(left: Color, right: Color, mut frac: (u32, std::num::NonZeroU32)) -> Color {
        if frac.0 > frac.1.into() {
            frac = (frac.1.into(), frac.1);
        }
        Color{
            r: Self::lerp_channel(left.r, right.r, frac),
            g: Self::lerp_channel(left.g, right.g, frac),
            b: Self::lerp_channel(left.b, right.b, frac),
            a: Self::lerp_channel(left.a, right.a, frac),
        }
    }

    pub fn alpha_over(over: Color, under: Color) -> Color {
        const MAX: u64 = u32::MAX as u64;
        const MAX_SQUARED: u64 = MAX * MAX;
        let over_alpha = over.a as u64;
        let under_alpha = under.a as u64;
        let over_inv_alpha = MAX - over_alpha;
        let under_inv_alpha = MAX - under_alpha;
        let inv_alpha = over_inv_alpha * under_inv_alpha;
        let alpha = (MAX_SQUARED - inv_alpha) / MAX;
        let rgb: [(u32, u32); 3] = [(over.r, under.r), (over.g, under.g), (over.b, under.b)];
        let rgb = rgb.iter().map(|&(over, under)| {
            let over_value = over as u64 * over_alpha;
            let under_value = under as u64 * over_inv_alpha;
            let under_int = under_value / MAX * under_alpha;
            let under_frac = under_value % MAX * under_alpha / MAX;
            let under_value = under_int + under_frac;
            (over_value / MAX + under_value / MAX + (over_value % MAX + under_value % MAX) / MAX) as u32
        }).collect::<Vec<u32>>();
        Color{
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
            a: alpha as u32,
        }
    }

    pub fn desaturate(&self) -> Color {
        let low = u32::min(self.r, u32::min(self.g, self.b));
        let high = u32::max(self.r, u32::max(self.g, self.b));
        let mid = Self::average_channel(low, high);
        Color{
            r: Self::average_channel(self.r, mid),
            g: Self::average_channel(self.g, mid),
            b: Self::average_channel(self.b, mid),
            a: self.a,
        }
    }

    fn average_channel(left: u32, right: u32) -> u32 {
        left / 2 + right / 2 + (left & right & 1)
    }

    fn lerp_channel(left: u32, right: u32, (numer, denom): (u32, std::num::NonZeroU32)) -> u32 {
        let denom: u32 = denom.into();
        let inv_numer = denom - numer;
        left / denom * inv_numer + right / denom * numer + (left % denom * inv_numer + right % denom * numer) / denom
    }
}
