use super::Image;

pub trait Parser {
    fn from_bytes(&self, iter: &mut dyn Iterator<Item=&u8>) -> Result<Box<dyn Image>, Box<dyn std::error::Error>>;
    fn to_bytes<'a>(&self, image: &'a dyn Image) -> Vec<u8>;
}
