mod backgrounds;
mod berries;
mod image;
mod savedata;

use backgrounds::{
    Metagenerator,
    TransFlagGen,
};

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
    Skull,
};

use image::{
    BmpParser,
    Color,
    Image,
    Parser,
    RGBA32Image,
};

use savedata::SaveLoader;

fn main() {
    let berries = SaveLoader::load_save("/home/leximainw/.local/share/Celeste/Saves/0.celeste").unwrap();
    let mut image: Box<dyn Image> = Box::new(RGBA32Image::new(120, 85));
    let trans = Color::from_srgba32(0);
    for x in 0..120 {
        for y in 0..85 {
            image.set_pixel((x, y), trans);
        }
    }
    let mut canvas = OpaqueCanvas::from_image(&mut *image);
    let mut text = TextField::new();
    let show_deaths = false;
    text.set_text(format!("{: >3}x", berries.red_berry_count()));
    text.draw(&mut canvas, 11, 9);
    let (death_text, death_offset) = if !show_deaths {
        ("".to_string(), 0)
    } else if berries.deaths >= 10000000 {
        ("".to_string(), 20)
    } else {
        let text = if berries.deaths >= 1000000 {
            format!("{}", berries.deaths)
        } else {
            format!("{}x", berries.deaths)
        };
        let len = text.len();
        (text, len * 2 + 9)
    };
    Berry.draw(&mut canvas, 29, 6);
    BerryRow::from_vec(berries.levels[0..3].iter().map(|x| x.berries.clone()).collect::<Vec<Vec<bool>>>()).draw(&mut canvas, 25 + death_offset, 22);
    BerryRow::from_vec(berries.levels[3..5].iter().map(|x| x.berries.clone()).collect::<Vec<Vec<bool>>>()).draw(&mut canvas, 28 + death_offset, 26);
    BerryRow::from_vec(berries.levels[6..8].iter().map(|x| x.berries.clone()).collect::<Vec<Vec<bool>>>()).draw(&mut canvas, 32 + death_offset, 30);
    if show_deaths {
        if death_offset == 20 {
            for i in 0..3 {
                Skull.draw(&mut canvas, i * 12 + 6, 20);
            }
        } else {
            text.set_text(death_text);
            text.draw(&mut canvas, 26 - death_offset, 23);
            Skull.draw(&mut canvas, 10 + death_offset, 20);
        }
    }
    WingedGoldBerry.draw(&mut *create_canvas(&mut *image, berries.ch1winged), 42, 5);
    MoonBerry.draw(&mut *create_canvas(&mut *image, berries.ch9moon), 78, 6);
    GoldBerry.draw(&mut *create_canvas(&mut *image, berries.ch9golden), 96, 4);
    for x in 0..8 {
        for y in 0..3 {
            GoldBerry.draw(&mut *create_canvas(&mut *image, berries.levels[x].goldens[y]), x * 14 + 5, y * 17 + 35);
        }
    }
    TransFlagGen::draw_under(&mut *image);
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
