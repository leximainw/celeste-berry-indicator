use super::{
    Image,
    Parser,
};

pub struct BmpParser;

impl Parser for BmpParser {
    fn from_bytes(&self, iter: &mut dyn Iterator<Item=&u8>) -> Result<Box<dyn Image>, Box<dyn std::error::Error>> {
        Err("reading .bmp files is not yet implemented".into())
    }

    fn to_bytes(&self, image: &dyn Image) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend_from_slice(b"BM");
        vec.extend_from_slice(&[0; 8]);
        vec.extend_from_slice(&54_i32.to_le_bytes());
        vec.extend_from_slice(&40_i32.to_le_bytes());
        vec.extend_from_slice(&(image.get_width() as u32).to_le_bytes());
        vec.extend_from_slice(&(image.get_height() as u32).to_le_bytes());
        vec.extend_from_slice(&1_i16.to_le_bytes());
        vec.extend_from_slice(&32_i16.to_le_bytes());
        vec.extend_from_slice(&[0; 8]);
        vec.extend_from_slice(&3780_i32.to_le_bytes());
        vec.extend_from_slice(&3780_i32.to_le_bytes());
        vec.extend_from_slice(&0_i32.to_le_bytes());
        vec.extend_from_slice(&[0; 4]);
        for y in (0..image.get_height()).rev() {
            for x in 0..image.get_width() {
                match image.get_pixel((x, y)).to_srgba32().to_be_bytes() {
                    [r, g, b, a] => {
                        vec.extend_from_slice(&[b, g, r, a]);
                    }
                }
            }
        }
        let size = (vec.len() as u32).to_le_bytes();
        for i in 0..4 {
            vec[i + 2] = size[i];
        }
        vec
    }
}
