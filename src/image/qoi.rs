use super::{
    Color,
    Image,
    Parser,
    RGBA32Image,
};

pub struct QoiParser;

impl QoiParser {
    fn match_slice(iter: &mut dyn Iterator<Item=&u8>, slice: &[u8]) -> bool {
        for byte in slice {
            if iter.next() != Some(byte) {
                return false;
            }
        }
        return true;
    }

    fn parse_image(iter: &mut dyn Iterator<Item=&u8>) -> Option<Box<dyn Image>> {
        let mut r: u8;
        let mut g: u8;
        let mut b: u8;
        let mut a = 255u8;
        let mut pixel: Color = Color::from_rgba32(255);
        let mut exit = false;
        let width = Self::read_u32(iter)?;
        let height = Self::read_u32(iter)?;
        let channels = iter.next()?;
        let mut index: usize = 0;
        let mut image = Box::new(RGBA32Image::new(width as usize, height as usize));
        loop {
            let value = iter.next();
            if let Some(&byte) = value {
                if byte == 0 {
                    if exit {
                        break;
                    } else {
                        exit = true;
                    }
                } else {
                    if exit {
                        exit = false;
                        // todo: emit hash pixel
                    }
                    if byte == 0xff {   // rgba
                        r = *iter.next()?;
                        g = *iter.next()?;
                        b = *iter.next()?;
                        a = *iter.next()?;
                        pixel = Color::from_rgba(r, g, b, a);
                        image.set_pixel_index(index, pixel);
                        index += 1;
                    } else if byte == 0xfe {   // rgb
                        r = *iter.next()?;
                        g = *iter.next()?;
                        b = *iter.next()?;
                        pixel = Color::from_rgba(r, g, b, a);
                        image.set_pixel_index(index, pixel);
                        index += 1;
                    } else if byte >= 0xc0 {   // run
                        let length = byte - 191;
                        for i in 0..length {
                            image.set_pixel_index(index, pixel);
                            index += 1;
                        }
                    } else if byte >= 0x80 {   // luma
                        let dg = byte as i32 - 160;
                        let byte = iter.next()?;
                        let dr_dg = (byte >> 4) as i32 - 8;
                        let db_dg = (byte & 15) as i32 - 8;
                    }
                }
            } else {
                None?;
            }
        }
        if !Self::match_slice(&mut *iter, b"\0\0\0\0\0\x01") {
            None?;
        }
        Some(image)
    }

    fn read_u32(iter: &mut dyn Iterator<Item=&u8>) -> Option<u32> {
        let mut value: u32 = 0;
        for i in 0..4 {
            if let Some(byte) = iter.next() {
                value <<= 8;
                value |= *byte as u32;
            } else {
                return None;
            }
        }
        Some(value)
    }
}

impl Parser for QoiParser {
    fn from_bytes(iter: &mut dyn Iterator<Item=&u8>) -> Result<Box<dyn Image>, Box<dyn std::error::Error>> {
        if !Self::match_slice(&mut *iter, b"qoif") {
            return Err("invalid QOI file".into());
        }
        Ok(Self::parse_image(&mut *iter).ok_or("unexpected end of file")?)
    }

    fn to_bytes<'a>(image: &'a dyn Image) -> &'a dyn Iterator<Item=&u8> {
        todo!();
    }
}
