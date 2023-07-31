pub struct BerryTracker {
    pub levels: [BerryTrackerLevel; 8],
    pub ch1winged: bool,
    pub ch9moon: bool,
    pub ch9golden: bool,
    pub deaths: usize,
}

pub struct BerryTrackerLevel {
    pub berries: Vec<bool>,
    pub goldens: [bool; 3],
    pub hearts: [bool; 3],
}

impl BerryTrackerLevel {
    fn new() -> BerryTrackerLevel {
        BerryTrackerLevel{
            berries: Vec::new(),
            goldens: [false; 3],
            hearts: [false; 3],
        }
    }
}

impl BerryTracker {
    pub fn new() -> BerryTracker {
        BerryTracker{
            levels: [
                BerryTrackerLevel::new(),
                BerryTrackerLevel::new(),
                BerryTrackerLevel::new(),
                BerryTrackerLevel::new(),
                BerryTrackerLevel::new(),
                BerryTrackerLevel::new(),
                BerryTrackerLevel::new(),
                BerryTrackerLevel::new(),
            ],
            ch1winged: false,
            ch9moon: false,
            ch9golden: false,
            deaths: 0,
        }
    }

    pub fn red_berry_count(&self) -> usize {
        self.levels.iter().map(|x| x.berries.iter().filter(|&&x| x).count()).sum()
    }
}
