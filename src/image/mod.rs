mod color;
mod image;
mod parser;
pub use color::Color as Color;
pub use image::Image as Image;
pub use parser::Parser as Parser;

mod rgba32image;
pub use rgba32image::RGBA32Image as RGBA32Image;

mod bmp;
pub use bmp::BmpParser as BmpParser;

mod qoi;
pub use qoi::QoiParser as QoiParser;
