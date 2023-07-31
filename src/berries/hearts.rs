use super::{
    Canvas,
    Drawable,
};

macro_rules! heart {
    ($name:ident, $light:literal, $dark:literal) => {
        pub struct $name;

        impl Drawable for $name {
            fn draw(&self, canvas: &mut dyn Canvas, x: usize, y: usize) {
                const BLK: u32 = 0x000000ff;
                const WHT: u32 = 0xffffffff;
                const LGT: u32 = $light;
                const DRK: u32 = $dark;
        
                <Self as Drawable>::draw_sprite(canvas, x, y, 14, 14, &[
                    0,   0, BLK, BLK, BLK,   0,   0,   0,   0, BLK, BLK, BLK,   0,   0,
                    0, BLK, LGT, LGT, LGT, BLK,   0,   0, BLK, LGT, LGT, LGT, BLK,   0,
                    BLK,LGT,LGT, DRK, DRK, LGT, BLK, BLK, LGT, DRK, WHT, WHT, LGT, BLK,
                    BLK,LGT,DRK, LGT, LGT, DRK, LGT, LGT, DRK, DRK, WHT, WHT, LGT, BLK,
                    BLK,LGT,DRK, LGT, LGT, DRK, DRK, DRK, WHT, DRK, DRK, DRK, LGT, BLK,
                    BLK,LGT,DRK, DRK, DRK, DRK, DRK, DRK, DRK, DRK, DRK, DRK, LGT, BLK,
                    BLK,LGT,LGT, DRK, LGT, DRK, DRK, DRK, DRK, DRK, DRK, LGT, LGT, BLK,
                    0, BLK, LGT, LGT, DRK, DRK, DRK, DRK, DRK, DRK, LGT, LGT, BLK,   0,
                    0, BLK, LGT, LGT, LGT, DRK, DRK, DRK, DRK, LGT, LGT, LGT, BLK,   0,
                    0,   0, BLK, WHT, LGT, LGT, LGT, LGT, LGT, LGT, WHT, BLK,   0,   0,
                    0,   0,   0, BLK, WHT, LGT, LGT, LGT, LGT, WHT, BLK,   0,   0,   0,
                    0,   0,   0,   0, BLK, WHT, LGT, LGT, WHT, BLK,   0,   0,   0,   0,
                    0,   0,   0,   0,   0, BLK, WHT, WHT, BLK,   0,   0,   0,   0,   0,
                    0,   0,   0,   0,   0,   0, BLK, BLK,   0,   0,   0,   0,   0,   0,
                ]);
            }
        }
    }
}

heart!(HeartA, 0x5caefaff, 0x2f5ffaff);
heart!(HeartB, 0xff2457ff, 0xb80b33ff);
heart!(HeartC, 0xfffc24ff, 0xb8a90bff);
