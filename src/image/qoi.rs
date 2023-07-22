use super::{
    Image,
    Parser,
    RGBA32Image,
};

pub struct QoiParser;

impl QoiParser {
    fn match_slice(iter: &mut dyn Iterator<Item=u8>, slice: &[u8]) -> bool {
        for byte in slice {
            if iter.next() != Some(*byte) {
                return false;
            }
        }
        return true;
    }
}

impl Parser for QoiParser {
    fn from_bytes(mut iter: Box<dyn Iterator<Item=u8>>) -> Result<Box<dyn Image>, Box<dyn std::error::Error>> {
        if !Self::match_slice(&mut *iter, b"qoif") {
            Err("invalid QOI file".into())
        } else {
            todo!()
        }
    }

    fn to_bytes<'a>(image: &'a dyn Image) -> &'a dyn Iterator<Item=u8> {
        todo!();
    }
}
