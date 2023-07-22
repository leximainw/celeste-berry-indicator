mod image;

use image::{
    Parser,
    QoiParser,
    RGBA32Image,
};

fn main() {
    println!("Hello, world!");

    let data = b"qoif\0\0\0\x04\0\0\0\x04\x04\0\xcf\0\0\0\0\0\0\0\x01";

    let image = QoiParser::from_bytes(&mut data.iter()).unwrap();

    println!("Image: {:?}", image.to_rgba32_image());
}
