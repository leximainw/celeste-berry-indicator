mod args;
mod backgrounds;
mod berries;
mod compression;
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
    HeartA,
    HeartB,
    HeartC,
    Skull,
};

use image::{
    Color,
    Image,
    RGBA32Image,
    Parser,
    BmpParser,
    QoiParser,
};

use savedata::{
    BerryTracker,
    SaveLoader,
};

const DEFAULT_OUTPUT: &str = "image.bmp";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(args) = args::parse_args() {
        let berries = if let Some(file) = &args.load_file {
            SaveLoader::load_save(file)
        } else {
            SaveLoader::load_save_id(args.load_id.unwrap_or_default())
        }?;
        let output = args.output_file.clone().unwrap_or_else(|| DEFAULT_OUTPUT.into());
        let extension = output.extension()
            .map(|x| x.to_str()).flatten()
            .map(|x| x.to_lowercase());
        let parser: Box<dyn Parser> = if let Some(extension) = extension {
            let extension = extension.as_str();
            match extension {
                "bmp" => Box::new(BmpParser),
                "qoi" => Box::new(QoiParser),
                _ => todo!(),
            }
        } else {
            todo!();
        };
        let mut image = render_berries(berries, args);
        image = Box::new(scale_image(&*image, 4));
        TransFlagGen.draw_under(&mut *image);
        std::fs::write(output, parser.to_bytes(&*image))?;
    }
    Ok(())
}

fn render_berries(berries: BerryTracker, args: args::Args) -> Box<dyn Image> {
    let mut image: Box<dyn Image> = Box::new(RGBA32Image::new(120, 85));
    let trans = Color::from_srgba32(0);
    for x in 0..120 {
        for y in 0..85 {
            image.set_pixel((x, y), trans);
        }
    }
    let mut canvas = OpaqueCanvas::from_image(&mut *image);
    let mut text = TextField::new();
    text.set_text(format!("{: >3}x", berries.red_berry_count()));
    text.draw(&mut canvas, 11, 9);
    let (death_text, death_offset) = if !args.show_deaths {
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
    let show_until = berries.levels.iter().position(|x| x.completed[0] == false).unwrap_or_else(|| 9);
    let has_any_goldens = berries.levels.iter().any(|x| x.goldens.iter().any(|x| *x));
    Berry.draw(&mut canvas, 29, 6);
    BerryRow::from_vec(berries.levels[0..usize::min(3, show_until)].iter()
        .map(|x| x.berries.clone()).collect::<Vec<Vec<bool>>>())
        .draw(&mut canvas, 25 + death_offset, 22);
    if show_until > 3 {
        BerryRow::from_vec(berries.levels[3..usize::min(5, show_until)].iter()
            .map(|x| x.berries.clone()).collect::<Vec<Vec<bool>>>())
            .draw(&mut canvas, 28 + death_offset, 26);
    }
    if show_until > 6 {
        BerryRow::from_vec(berries.levels[6..usize::min(8, show_until)].iter()
            .map(|x| x.berries.clone()).collect::<Vec<Vec<bool>>>())
            .draw(&mut canvas, 32 + death_offset, 30);
    }
    if args.show_deaths {
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
    if !args.hide_incomplete || berries.ch1winged {
        WingedGoldBerry.draw(&mut *create_canvas(&mut *image, berries.ch1winged), 42, 5);
    }
    if !args.hide_incomplete || berries.ch9moon {
        MoonBerry.draw(&mut *create_canvas(&mut *image, berries.ch9moon), 78, 6);
    }
    if !args.hide_incomplete || (berries.ch9completed && has_any_goldens) || berries.ch9golden {
        GoldBerry.draw(&mut *create_canvas(&mut *image, berries.ch9golden), 96, 4);
    }
    let completed_8b = berries.levels[7].completed[1];
    for y in 0..3 {
        let show_heart = berries.levels.iter().map(|x|
            args.show_hearts && !x.goldens[y] && (!x.hearts[y] || !has_any_goldens)
                && (!args.hide_incomplete || x.completed[y] || x.hearts[y])
        ).collect::<Vec<bool>>();
        let show_berry = berries.levels.iter().enumerate().map(|(i, x)|
            !show_heart[i] && (!args.hide_incomplete || (completed_8b && x.completed[y]) || x.goldens[y])
        ).collect::<Vec<bool>>();
        let mut offset = if args.space_hearts {
            4 - (show_heart.iter().fold((0, false), |a, &x| (a.0 + if x && a.1 { 1 } else { 0 }, x)).0 + 1) / 2
        } else { 4 };
        let mut pushing = false;
        for x in 0..8 {
            if show_heart[x] {
                let has_heart = berries.levels[x].hearts[y];
                if pushing && args.space_hearts {
                    offset += 1;
                } else {
                    pushing = true;
                }
                match y {
                    0 => HeartA.draw(&mut *create_canvas(&mut *image, has_heart), x * 14 + offset, y * 17 + 35),
                    1 => HeartB.draw(&mut *create_canvas(&mut *image, has_heart), x * 14 + offset, y * 17 + 35),
                    2 => HeartC.draw(&mut *create_canvas(&mut *image, has_heart), x * 14 + offset, y * 17 + 35),
                    _ => panic!("y was {y}, but should be 0..3"),
                }
            } else {
                if pushing {
                    pushing = false;
                }
                if show_berry[x] {
                    GoldBerry.draw(&mut *create_canvas(&mut *image, berries.levels[x].goldens[y]), x * 14 + 1 + offset, y * 17 + 35);
                }
            }
        }
    }
    image
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
