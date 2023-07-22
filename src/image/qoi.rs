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
        let mut r = 0u8;
        let mut g = 0u8;
        let mut b = 0u8;
        let mut a = 255u8;
        let mut pixel: Color = Color::from_rgba32_linear(255);
        let mut exit = false;
        let width = Self::read_u32(iter)?;
        let height = Self::read_u32(iter)?;
        let channels = iter.next()?;
        let colorspace = iter.next()?;
        let mut index: usize = 0;
        let mut image = Box::new(RGBA32Image::new(width as usize, height as usize));
        let mut table: [Color; 64] = [Color::ZERO; 64];
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
                        image.set_pixel_index(index, pixel);
                        index += 1;
                    }
                    if byte == 0xff {   // rgba
                        r = *iter.next()?;
                        g = *iter.next()?;
                        b = *iter.next()?;
                        a = *iter.next()?;
                    } else if byte == 0xfe {   // rgb
                        r = *iter.next()?;
                        g = *iter.next()?;
                        b = *iter.next()?;
                    } else if byte >= 0xc0 {   // run
                        let length = byte - 191;
                        for i in 0..length {
                            image.set_pixel_index(index, pixel);
                            index += 1;
                        }
                        continue;
                    } else if byte >= 0x80 {   // luma
                        let dg = byte - 160;
                        let byte = iter.next()?;
                        let dr = (byte >> 4) - 8 + dg;
                        let db = (byte & 15) - 8 + dg;
                        r += dr;
                        g += dg;
                        b += db;
                    } else if byte >= 0x40 {   // diff
                        r += (byte >> 4 & 3) - 2;
                        g += (byte >> 2 & 3) - 2;
                        b += (byte & 3) - 2;
                    } else {
                        pixel = table[byte as usize];
                        image.set_pixel_index(index, pixel);
                        index += 1;
                        continue;
                    }
                    match colorspace {
                        0 => pixel = Color::from_srgba(r, g, b, a),
                        1 => pixel = Color::from_rgba_linear(r, g, b, a),
                        _ => None?,
                    }
                    table[((r * 3 + g * 5 + b * 7 + a * 11) % 64) as usize] = pixel;
                    image.set_pixel_index(index, pixel);
                    index += 1;
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
