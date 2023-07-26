use super::BerryTracker;

pub struct SaveLoader;

impl SaveLoader {
    const BERRY_IDS: [&'static [&'static str]; 8] = [
        &[
            "2:11", "3:9", "3b:2", "5z:10", "5:21", "5a:2",
            "7zb:2", "6:12", "s1:9", "7z:3", "8zb:1", "7a:12", "9z:3", "8b:1", "9:14",
            "10zb:1", "11:9", "9b:9", "9c:2", "12z:8",
        ],
        &[
            "d3:10", "d2:9", "d1:67", "d6:2", "d0:6", "d4:6", "d5:12", "d2:31", "1:1",
            "4:4", "5:15", "8:18", "9b:5", "9:22", "10:27", "12c:7", "12d:44",
            "end_3c:13",
        ],
        &[
            "s2:18", "s2:6", "00-a:5", "00-b:42", "s3:2", "04-b:14", "06-a:7", "06-b:14", "05-c:2", "06-c:3", "07-b:4",
            "12-c:1", "11-d:52", "13-b:31", "13-x:13", "12-y:1", "10-y:2", "08-x:4",
            "06-d:238", "04-c:40", "03-b:1", "03-b:25",
            "roof03:97", "roof06:276", "roof06:308",
        ],
        &[
            "a-01x:11", "a-02:8", "a-03:33", "a-04:11", "a-06:6", "a-07:16", "a-10:13", "a-09:12",
            "b-01:6", "b-01:13", "b-02:20", "b-03:5", "b-07:15", "b-04:1", "b-02:58", "b-secb:9", "b-08:11",
            "c-01:26", "c-00:17", "c-05:21", "c-06b:43", "c-06:35", "c-08:28", "c-10:55",
            "d-00b:11", "d-01:7", "d-04:88", "d-07:70", "d-09:18",
        ],
        &[
            "a-00x:7", "a-01:256", "a-01:164", "a-04:2", "a-03:4", "a-02:23", "a-05:22", "a-07:6", "a-06:2", "a-14:12", "a-11:2", "a-15:182",
            "b-18:2", "b-01c:85", "b-21:99", "b-20:183", "b-20:72", "b-03:24", "b-05:23", "b-10:4", "b-12:3", "b-17:14", "b-17:10",
            "c-08:112",
            "d-04:122", "d-04:16", "d-13:157", "d-15:217", "d-15:335", "d-19:533",
            "e-06:56",
        ],
        &[],
        &[
            "a-02b:61", "a-04b:136", "a-04b:85", "a-05:54",
            "b-02:101", "b-02b:102", "b-02e:112", "b-04:67", "b-08:129", "b-09:167",
            "c-03b:228", "c-05:248", "c-06b:281", "c-07b:291", "c-08:331", "c-09:354",
            "d-00:43", "d-01c:226", "d-01d:282", "d-03:383", "d-04:388", "d-07:484", "d-08:527", "d-10b:682",
            "e-02:7", "e-05:237", "e-07:473", "e-09:398", "e-12:504", "e-11:425", "e-10:515", "e-13:829",
            "f-01:639", "f-00:590", "f-07:711", "f-08b:856", "f-08c:759", "f-11:1068", "f-11:1229", "f-11:1238",
            "g-00b:37", "g-00b:127", "g-00b:114", "g-01:66", "g-01:279", "g-01:342", "g-03:1504",
        ],
        &[
            "b-06:174",
            "c-00b:211", "c-02:248", "c-03b:276",
            "d-06:130",
        ],
    ];

    const GOLDEN_IDS: [&'static [&'static str]; 8] = [
        &[ "1:12", "00:25", "00:50", ],
        &[ "start:5", "start:5", "00:6", ],
        &[ "s0:7", "00:2", "00:86", ],
        &[ "a-00:13", "a-00:41", "00:1", ],
        &[ "a-00b:3", "start:3", "00:25", ],
        &[ "00:51", "a-00:137", "00:3", ],
        &[ "a-00:57", "a-00:102", "01:334", ],
        &[ "a-00:19", "a-00:22", "00:93", ],
    ];

    const CH1_WINGED: &'static str = "end:4";
    const CH9_MOON: &'static str = "j-19:9";
    const CH9_GOLDEN: &'static str = "a-00:449";

    pub fn load_save(file: &str) -> Result<BerryTracker, Box<dyn std::error::Error>> {
        Ok(Self::load_data(std::fs::read_to_string(file)?))
    }

    pub fn load_data(xml: String) -> BerryTracker {
        let mut berries = BerryTracker::new();
        for (idx, berry_ids) in Self::BERRY_IDS.iter().enumerate() {
            berries.levels[idx].berries.extend(berry_ids.iter().map(|x| false))
        }
        let areas = Self::find_tag(&xml, "Areas");
        if let Some(areas) = areas.0 {
            let mut remain = areas.1;
            loop {
                let area = Self::find_tag(&remain, "AreaStats");
                remain = area.2;
                if let Some(area) = area.0 {
                    let area_idx = match Self::find_attr(&area.0, "ID") {
                        Some(value) => match value.parse() {
                            Ok(value) => value,
                            Err(_) => 0,
                        },
                        None => 0,
                    };
                    let modes = Self::find_tag(&area.1, "Modes");
                    if let Some(modes) = modes.0 {
                        let mut remain = modes.1;
                        let mut mode_idx = 0;
                        loop {
                            let mode = Self::find_tag(&remain, "AreaModeStats");
                            remain = mode.2;
                            if let Some(mode) = mode.0 {
                                let strawbs = Self::find_tag(&mode.1, "Strawberries");
                                if let Some(strawbs) = strawbs.0 {
                                    let mut remain = strawbs.1;
                                    loop {
                                        let strawb = Self::find_tag(&remain, "EntityID");
                                        remain = strawb.2;
                                        if let Some(strawb) = strawb.0 {
                                            if let Some(key) = Self::find_attr(strawb.0, "Key") {
                                                if area_idx == 1 && mode_idx == 0 && key == Self::CH1_WINGED {
                                                    berries.ch1winged = true;
                                                } else if area_idx == 10 && mode_idx == 0 && key == Self::CH9_MOON {
                                                    berries.ch9moon = true;
                                                } else if area_idx == 10 && mode_idx == 0 && key == Self::CH9_GOLDEN {
                                                    berries.ch9golden = true;
                                                } else if (1..8).contains(&area_idx) || area_idx == 9 {
                                                    let area_idx = if area_idx == 9 { 7 } else { area_idx - 1 };
                                                    if (0..3).contains(&mode_idx) && Self::GOLDEN_IDS[area_idx][mode_idx] == key {
                                                        berries.levels[area_idx].goldens[mode_idx] = true;
                                                    } else if mode_idx == 0 {
                                                        for (i, id) in Self::BERRY_IDS[area_idx].iter().enumerate() {
                                                            if id == &key {
                                                                berries.levels[area_idx].berries[i] = true;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            } else {
                                break;
                            }
                            mode_idx += 1;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        berries
    }

    fn find_tag<'a, 'b>(xml: &'a str, tag_type: &'b str) -> (Option<(&'a str, &'a str)>, &'a str, &'a str) {
        if let Some((left, center)) = xml.split_once(&format!("<{tag_type}>")) {
            if let Some((center, right)) = center.split_once(&format!("</{tag_type}>")) {
                (Some((&center[..0], center)), left, right)
            } else {
                (None, xml, &xml[xml.len()..])
            }
        } else if let Some((left, bulk)) = xml.split_once(&format!("<{tag_type} ")) {
            if let Some((tag, center)) = bulk.split_once(">") {
                if tag.is_char_boundary(tag.len() - 1) && &tag[tag.len() - 1..] == "/" {
                    (Some((&tag[..tag.len() - 1], &tag[tag.len()..])), left, center)
                } else if let Some((center, right)) = center.split_once(&format!("</{tag_type}>")) {
                    (Some((tag, center)), left, right)
                } else {
                    (None, xml, &xml[xml.len()..])
                }
            } else {
                (None, xml, &xml[xml.len()..])
            }
        } else {
            (None, xml, &xml[xml.len()..])
        }
    }

    fn find_attr<'a, 'b>(tag: &'a str, attr_name: &'b str) -> Option<&'a str> {
        if let Some((_, text)) = tag.split_once(&format!("{attr_name}=\"")) {
            if let Some((text, _)) = text.split_once('"') {
                Some(text)
            } else {
                None
            }
        } else {
            None
        }
    }
}
