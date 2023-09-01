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
        let mut r = 0_u8;
        let mut g = 0_u8;
        let mut b = 0_u8;
        let mut a = 255_u8;
        let mut pixel: Color = Color::from_rgba32_linear(255);
        let mut exit = false;
        let width = Self::read_u32(iter)?;
        let height = Self::read_u32(iter)?;
        let _channels = iter.next()?;
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
                        for _ in 0..length {
                            image.set_pixel_index(index, pixel);
                            index += 1;
                        }
                        table[((r * 3 + g * 5 + b * 7 + a * 11) % 64) as usize] = pixel;
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
        for _ in 0..4 {
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
    fn from_bytes(&self, iter: &mut dyn Iterator<Item=&u8>) -> Result<Box<dyn Image>, Box<dyn std::error::Error>> {
        if !Self::match_slice(&mut *iter, b"qoif") {
            return Err("invalid QOI file".into());
        }
        Ok(Self::parse_image(&mut *iter).ok_or("unexpected end of file")?)
    }

    fn to_bytes(&self, image: &dyn Image) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend_from_slice(b"qoif");
        vec.extend_from_slice(&(image.get_width() as u32).to_be_bytes());
        vec.extend_from_slice(&(image.get_height() as u32).to_be_bytes());
        vec.push(4);   // TODO: support RGB format
        vec.push(0);   // TODO: support linear colorspace
        let size = image.get_width() * image.get_height();
        let mut run = 0_u8;
        let mut pixel = 255_u32;
        let mut last_pixel;
        let mut table: [u32; 64] = [0; 64];
        for i in 0..size {
            last_pixel = pixel;
            pixel = image.get_pixel_index(i).to_srgba32();
            if pixel == last_pixel {   // run
                if last_pixel == 255 {
                    table[53] = 255;
                }
                run += 1;
                if run == 62 {
                    vec.push(253);
                    run = 0;
                }
                continue;
            } else if run != 0 {
                vec.push(run + 191);
                run = 0;
            }
            match pixel.to_be_bytes() {
                [r, g, b, a] => {
                    let index = ((r * 3 + g * 5 + b * 7 + a * 11) % 64) as usize;
                    if table[index] == pixel {   // index
                        vec.push(index as u8);
                        continue;
                    }
                    table[index] = pixel;
                    match last_pixel.to_be_bytes() {
                        [lr, lg, lb, la] => {
                            if a != la {   // rgba
                                vec.extend_from_slice(&[255, r, g, b, a]);
                                continue;
                            }
                            let dr = r as i32 - lr as i32;
                            let dg = g as i32 - lg as i32;
                            let db = b as i32 - lb as i32;
                            if (-2..2).contains(&dr) && (-2..2).contains(&dg) && (-2..2).contains(&db) {   // diff
                                let dr = (dr + 2) as u8;
                                let dg = (dg + 2) as u8;
                                let db = (db + 2) as u8;
                                vec.push(64 | dr << 4 | dg << 2 | db);
                                continue;
                            }
                            let dr_dg = dr - dg;
                            let db_dg = db - dg;
                            if (-32..32).contains(&dg) && (-8..8).contains(&dr_dg) && (-8..8).contains(&db_dg) {   // luma
                                let dg = (dg + 32) as u8;
                                let dr_dg = (dr_dg + 8) as u8;
                                let db_dg = (db_dg + 8) as u8;
                                vec.push(128 | dg);
                                vec.push(dr_dg << 4 | db_dg);
                                continue;
                            }
                            vec.extend_from_slice(&[254, r, g, b]);   // rgb
                        }
                    }
                }
            }
        }
        if run != 0 {
            vec.push(run + 192);
        }
        vec.extend_from_slice(b"\0\0\0\0\0\0\0\x01");
        vec
    }
}
