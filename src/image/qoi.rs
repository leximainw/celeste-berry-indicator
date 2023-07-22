use super::{
    Image,
    Parser,
    RGBA32Image,
};

pub struct QoiParser;

impl Parser for QoiParser {
    fn from_bytes(iter: &dyn Iterator<Item=u8>) -> Box<dyn Image> {
        todo!();
    }

    fn to_bytes<'a>(image: &'a dyn Image) -> &'a dyn Iterator<Item=u8> {
        todo!();
    }
}
