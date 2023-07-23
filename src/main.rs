mod image;

use image::{
    Parser,
    QoiParser,
};

fn main() {
    println!("Hello, world!");

    // let data = b"qoif\0\0\0\x04\0\0\0\x04\x04\0\xcf\0\0\0\0\0\0\0\x01";
    let data = b"qoif\0\0\0\x04\0\0\0\x04\x04\0\xc3\xfe\xff\xff\xff\xc2\xfe\0\0\0\xc2\xfe\xff\xff\xff\xc2\0\0\0\0\0\0\0\x01";

    let image = QoiParser::from_bytes(&mut data.iter()).unwrap();

    println!("Image: {:?}", image.to_rgba32_image());

    let image = QoiParser::from_bytes(&mut data.iter()).unwrap();

    println!("Bytes: {:?}", QoiParser::to_bytes(&*image));
}
