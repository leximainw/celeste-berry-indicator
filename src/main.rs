mod berries;
mod image;

use berries::{
    Canvas,
    Drawable,
    FadedCanvas,
    OpaqueCanvas,
    GoldBerry,
    WingedGoldBerry,
};

use image::{
    BmpParser,
    Parser,
    QoiParser,
};

const LEVEL_BERRIES: [[bool; 8]; 3] = [
    [  true,  true,  true,  true,  true, false, false, false, ],
    [ false, false, false, false, false, false, false, false, ],
    [  true,  true, false, false, false, false, false, false, ],
];

fn main() {
    let data = b"qoif\0\0\0\x78\0\0\0\x55\x04\0\xfe\x5b\xce\xfa\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\xfe\xf5\xa9\xb8\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\xfe\xff\xff\xff\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\x29\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\x22\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\0\0\0\0\0\0\0\x01";
    let mut image = QoiParser::from_bytes(&mut data.iter()).unwrap();
    let mut canvas = OpaqueCanvas::from_image(&mut *image);
    WingedGoldBerry::draw(&mut canvas, 42, 5);
    let mut canvas = FadedCanvas::from_image(&mut *image);
    GoldBerry::draw(&mut canvas, 96, 4);
    for x in 0..8 {
        for y in 0..3 {
            let mut canvas: Box<dyn Canvas> = if LEVEL_BERRIES[y][x] {
                Box::new(OpaqueCanvas::from_image(&mut *image)) as Box<dyn Canvas>
            } else {
                Box::new(FadedCanvas::from_image(&mut *image)) as Box<dyn Canvas>
            };
            GoldBerry::draw(&mut *canvas, x * 14 + 5, y * 17 + 35);
        }
    }
    std::fs::write("image.bmp", BmpParser::to_bytes(&*image)).unwrap();
}
