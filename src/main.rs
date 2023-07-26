mod berries;
mod image;
mod savedata;

use berries::{
    Canvas,
    Drawable,
    FadedCanvas,
    OpaqueCanvas,
    TextField,
    BerryRow,
    Berry,
    GoldBerry,
    WingedGoldBerry,
    MoonBerry,
};

use image::{
    BmpParser,
    Image,
    Parser,
    QoiParser,
    RGBA32Image,
};

use savedata::SaveLoader;

fn main() {
    let berries = SaveLoader::load_save("/home/leximainw/.local/share/Celeste/Saves/0.celeste").unwrap();
    let data = b"qoif\0\0\0\x78\0\0\0\x55\x04\0\xfe\x5b\xce\xfa\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\xfe\xf5\xa9\xb8\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\xfe\xff\xff\xff\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\x29\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\x22\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xfd\xf6\0\0\0\0\0\0\0\x01";
    let mut image = QoiParser::from_bytes(&mut data.iter()).unwrap();
    let mut canvas = OpaqueCanvas::from_image(&mut *image);
    let mut text = TextField::new();
    text.set_text(format!("{: >3}x", berries.red_berry_count()));
    text.draw(&mut canvas, 11, 9);
    Berry.draw(&mut canvas, 29, 6);
    BerryRow::from_vec(berries.levels[0..3].iter().map(|x| x.berries.clone()).collect::<Vec<Vec<bool>>>()).draw(&mut canvas, 25, 22);
    BerryRow::from_vec(berries.levels[3..5].iter().map(|x| x.berries.clone()).collect::<Vec<Vec<bool>>>()).draw(&mut canvas, 28, 26);
    BerryRow::from_vec(berries.levels[6..8].iter().map(|x| x.berries.clone()).collect::<Vec<Vec<bool>>>()).draw(&mut canvas, 32, 30);
    WingedGoldBerry.draw(&mut *create_canvas(&mut *image, berries.ch1winged), 42, 5);
    MoonBerry.draw(&mut *create_canvas(&mut *image, berries.ch9moon), 78, 6);
    GoldBerry.draw(&mut *create_canvas(&mut *image, berries.ch9golden), 96, 4);
    for x in 0..8 {
        for y in 0..3 {
            GoldBerry.draw(&mut *create_canvas(&mut *image, berries.levels[x].goldens[y]), x * 14 + 5, y * 17 + 35);
        }
    }
    std::fs::write("image.bmp", BmpParser::to_bytes(&scale_image(&*image, 4))).unwrap();
}

fn create_canvas(image: &mut dyn Image, active: bool) -> Box<dyn Canvas + '_> {
    if active {
        Box::new(OpaqueCanvas::from_image(image)) as Box<dyn Canvas>
    } else {
        Box::new(FadedCanvas::from_image(image)) as Box<dyn Canvas>
    }
}

fn scale_image(image: &dyn Image, scale: usize) -> RGBA32Image {
    let mut scaled = RGBA32Image::new(image.get_width() * scale, image.get_height() * scale);
    if scale == 0 {
        return scaled;
    }
    for x in 0..image.get_width() {
        for y in 0..image.get_height() {
            let color = image.get_pixel((x, y));
            for u in 0..scale {
                for v in 0..scale {
                    scaled.set_pixel((x * scale + u, y * scale + v), color);
                }
            }
        }
    }
    scaled
}
