use crate::image::Color as Color;

mod generator;
pub use generator::Metagenerator as Metagenerator;
pub use generator::Generator as Generator;

mod flag_stripes;
pub use flag_stripes::FlagStripes as FlagStripes;

mod trans;
pub use trans::FlagGenerator as TransFlagGen;
pub use trans::Flag as TransFlag;
