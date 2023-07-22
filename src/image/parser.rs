use super::Image;

pub trait Parser {
    fn from_bytes(iter: &mut dyn Iterator<Item=&u8>) -> Result<Box<dyn Image>, Box<dyn std::error::Error>>;
    fn to_bytes<'a>(image: &'a dyn Image) -> &'a dyn Iterator<Item=&u8>;
}
