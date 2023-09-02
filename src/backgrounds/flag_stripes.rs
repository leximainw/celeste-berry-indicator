use super::Color;

pub struct FlagStripes;

impl FlagStripes {
    pub fn stripes(stripes: &[(u32, usize)], pos: usize, size: usize) -> Color {
        let weight: usize = stripes.iter().map(|x| x.1).sum();
        let lower_block = pos * weight / size;
        let upper_block = (pos + 1) * weight / size;
        if lower_block == upper_block || lower_block + 1 == weight {
            Color::from_srgba32(stripes[lower_block].0)
        } else if lower_block + 1 == upper_block {
            let lower_weight = pos * size % weight;
            Color::lerp(Color::from_srgba32(stripes[lower_block].0),
                Color::from_srgba32(stripes[upper_block].0),
                (lower_weight as u32, std::num::NonZeroU32::new(weight as u32).unwrap()))
        } else {
            todo!();
        }
    }
}