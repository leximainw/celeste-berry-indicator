pub struct BerryTracker {
    pub levels: [BerryTrackerLevel; 8],
    pub ch1winged: bool,
    pub ch9moon: bool,
    pub ch9golden: bool,
}

pub struct BerryTrackerLevel {
    pub berries: Vec<bool>,
    pub goldens: [bool; 3],
}

impl BerryTracker {
    pub fn red_berry_count(&self) -> usize {
        self.levels.iter().map(|x| x.berries.iter().filter(|&&x| x).count()).sum()
    }
}
