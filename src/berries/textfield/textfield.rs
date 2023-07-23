use super::{
    Canvas,
    Drawable,
};

pub struct TextField {
    text: String,
}

impl TextField {
    pub fn new() -> TextField {
        TextField{
            text: String::new(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Drawable for TextField {
    fn draw(&self, canvas: &mut dyn Canvas, mut x: usize, y: usize) {
        for c in self.text.chars() {
            match c {
                '1' => super::char_31::Char.draw(canvas, x + 1, y),
                '5' => super::char_35::Char.draw(canvas, x, y),
                '7' => super::char_37::Char.draw(canvas, x, y),
                'x' => super::char_78::Char.draw(canvas, x, y + 2),
                _ => panic!("no sprite for '{c}'"),
            }
            x += 4;
        }
    }
}
