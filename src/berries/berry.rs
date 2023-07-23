use super::{
    Canvas,
    Drawable,
};

pub struct Strawberry;

impl Drawable for Strawberry {
    fn draw(&self, canvas: &mut dyn Canvas, x: usize, y: usize) {
        const BLACK: u32 = 0x000000ff;
        const GREN0: u32 = 0x37946eff;
        const GREN1: u32 = 0x6abe30ff;
        const PURPL: u32 = 0x491675ff;
        const  RED0: u32 = 0x8a0f36ff;
        const  RED1: u32 = 0xde2a2aff;
        const  RED2: u32 = 0xff5f42ff;

        <Self as Drawable>::draw_sprite(canvas, x, y, 10, 13, &[
                0,     0,     0,     0, BLACK, BLACK,     0,     0,     0,     0,
                0,     0,     0, BLACK, BLACK, GREN1, BLACK,     0,     0,     0,
                0,     0, BLACK, GREN1, BLACK, GREN0, GREN1, BLACK,     0,     0,
                0, BLACK, GREN1, PURPL, GREN1, GREN1, PURPL, BLACK, BLACK,     0,
                0, BLACK, PURPL,  RED0, PURPL, PURPL,  RED0, PURPL, BLACK,     0,
            BLACK,  RED0,  RED1,  RED1,  RED1,  RED1,  RED2,  RED0,  RED0, BLACK,
            BLACK,  RED2,  RED1,  RED1,  RED2,  RED1,  RED1,  RED1,  RED2, BLACK,
            BLACK,  RED0,  RED1,  RED1,  RED0,  RED1,  RED2,  RED1,  RED0, BLACK,
            BLACK,  RED1,  RED0,  RED1,  RED1,  RED1,  RED0,  RED0,  RED1, BLACK,
                0, BLACK,  RED1,  RED0,  RED0,  RED0,  RED0,  RED1, BLACK,     0,
                0,     0, BLACK,  RED1,  RED0,  RED0,  RED1, BLACK,     0,     0,
                0,     0,     0, BLACK,  RED1,  RED1, BLACK,     0,     0,     0,
                0,     0,     0,     0, BLACK, BLACK,     0,     0,     0,     0,
        ]);
    }
}
