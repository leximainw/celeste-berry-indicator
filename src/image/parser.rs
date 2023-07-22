use super::Image;

pub trait Parser {
    fn from_bytes(iter: &dyn Iterator<Item=u8>) -> Box<dyn Image>;
    fn to_bytes<'a>(image: &'a dyn Image) -> &'a dyn Iterator<Item=u8>;
}
