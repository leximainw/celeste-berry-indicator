use std::path::PathBuf;

const DEFAULT_EXTENSION: &str = "bmp";

pub struct Args {
    pub background: String,
    pub hide_incomplete: bool,
    pub load_file: Option<PathBuf>,
    pub load_id: Option<usize>,
    pub output_file: Option<PathBuf>,
    pub show_deaths: bool,
    pub show_hearts: bool,
    pub space_hearts: bool,
}

impl Args {
    pub fn new() -> Args {
        Args{
            background: "trans".to_string(),
            hide_incomplete: false,
            load_file: None,
            load_id: None,
            output_file: None,
            show_deaths: false,
            show_hearts: true,
            space_hearts: false,
        }
    }
}

pub fn parse_args() -> Option<Args> {
    parse_args_core(&mut std::env::args().skip(1))
}

#[cfg(not(target_os = "windows"))]
fn parse_args_core(iter: &mut dyn Iterator<Item=String>) -> Option<Args> {
    let mut args = Args::new();
    for arg in iter {
        let mut chars = arg.chars();
        if chars.next() == Some('-') {
            if chars.next() == Some('-') {
                let arg = arg.to_lowercase();
                if arg.starts_with("--bg=") {
                    args.background = arg[5..].to_string();
                } else if arg == "--help" {
                    println!("--bg={{bg}} - - - - render the image with a specified background");
                    println!("--help  - - - - - print this list");
                    println!("--help:bg - - - - print a list of available backgrounds");
                    println!("--deaths  - - - - render death count");
                    println!("--id={{0..2}} - - - select save file by ID");
                    println!("--no-hearts - - - don't render hearts");
                    println!("--no-spoilers - - hide uncollected items from incomplete levels");
                    println!("--output={{file}} - write to specified file");
                    println!("--spacing - - - - add space between adjacent hearts");
                    return None;
                } else if arg.starts_with("--help:") {
                    let category = &arg[7..];
                    match category {
                        "bg" => {
                            println!("--bg=trans  - - transgender flag (default)");
                            println!("--bg=aroace - - aroace flag");
                            println!("--bg=aro  - - - aromantic flag");
                            println!("--bg=ace  - - - asexual flag");
                            println!("--bg=bi - - - - bisexual flag");
                            println!("--bg=enby - - - nonbinary flag");
                            println!("--bg=gay  - - - gay (men loving men) flag");
                            println!("--bg=lesbian  - lesbian (women loving women) flag");
                            println!("--bg=lesbian5 - lesbian flag (5-stripe version)");
                            println!("--bg=queer  - - queer flag");
                            println!("--bg=intersex - intersex flag");
                            println!("--bg=rainbow  - rainbow flag (6-stripe version)");
                            println!("--bg=old-progress - progress pride flag");
                            println!("--bg=progress - intersex progress pride flag");
                        },
                        _ => println!("unknown category {category}"),
                    }
                    return None;
                } else if arg == "--deaths" {
                    args.show_deaths = true;
                } else if arg.starts_with("--id=") {
                    if let Ok(id) = str::parse::<usize>(&arg[5..]) {
                        args.load_id = Some(id);
                    }
                } else if arg == "--no-hearts" {
                    args.show_hearts = false;
                } else if arg == "--no-spoilers" {
                    args.hide_incomplete = true;
                } else if arg.starts_with("--output=") {
                    let mut output_file: PathBuf = arg[9..].into();
                    if output_file.extension() == None {
                        output_file.set_extension(DEFAULT_EXTENSION);
                    }
                    args.output_file = Some(output_file);
                } else if arg == "--spacing" {
                    args.space_hearts = true;
                }
            }
        } else if args.load_file == None {
            args.load_file = Some(arg.into());
        } else if args.output_file == None {
            args.output_file = Some(arg.into());
        }
    }
    if args.output_file == None {
        if let Some(load_file) = args.load_file.clone() {
            if load_file.extension()
                    .map(|x| x.to_str()).flatten()
                    .map(|x| x.to_lowercase())
                    != Some("celeste".to_string()) {
                args.output_file = Some(load_file);
                args.load_file = None;
            }
        }
    }
    Some(args)
}

#[cfg(target_os = "windows")]
fn parse_args_core(iter: &mut dyn Iterator<Item=String>) -> Option<Args> {
    let mut args = Args::new();
    for arg in iter {
        let mut chars = arg.chars();
        if chars.next() == Some('/') {
            let arg = arg.to_lowercase();
            if arg.starts_with("/bg=") {
                args.background = arg[4..].to_string();
            } if arg == "/help" {
                println!("/bg={{bg}} - - - - render the image with a specified background");
                println!("/help  - - - - - print this list");
                println!("/help:bg - - - - print a list of available backgrounds");
                println!("/deaths  - - - - render death count");
                println!("/id={{0..2}} - - - select save file by ID");
                println!("/no-hearts - - - don't render hearts");
                println!("/no-spoilers - - hide uncollected items from incomplete levels");
                println!("/output={{file}} - write to specified file");
                println!("/spacing - - - - add space between adjacent hearts");
                return None;
            } else if arg.starts_with("/help:") {
                let category = &arg[6..];
                match category {
                    "bg" => {
                        println!("/bg=trans  - - transgender flag (default)");
                        println!("/bg=aroace - - aroace flag");
                        println!("/bg=aro  - - - aromantic flag");
                        println!("/bg=ace  - - - asexual flag");
                        println!("/bg=bi - - - - bisexual flag");
                        println!("/bg=enby - - - nonbinary flag");
                        println!("/bg=gay  - - - gay (men loving men) flag");
                        println!("/bg=lesbian  - lesbian (women loving women) flag");
                        println!("/bg=lesbian5 - lesbian flag (5-stripe version)");
                        println!("/bg=queer  - - queer flag");
                        println!("/bg=rainbow  - rainbow flag (6-stripe version)");
                    },
                    _ => println!("unknown category {category}"),
                };
                return None;
            } else if arg == "/deaths" {
                args.show_deaths = true;
            } else if arg.starts_with("/id=") {
                if let Ok(id) = str::parse::<usize>(&arg[4..]) {
                    args.load_id = Some(id);
                }
            } else if arg == "/no-hearts" {
                args.show_hearts = false;
            } else if arg == "/no-spoilers" {
                args.hide_incomplete = true;
            } else if arg.starts_with("/output=") {
                let mut output_file: PathBuf = arg[8..].into();
                if output_file.extension() == None {
                    output_file.set_extension(DEFAULT_EXTENSION);
                }
                args.output_file = Some(output_file);
            } else if arg == "/spacing" {
                args.space_hearts = true;
            }
        } else if args.load_file == None {
            args.load_file = Some(arg.into());
        } else if args.output_file == None {
            args.output_file = Some(arg.into());
        }
    }
    if args.output_file == None {
        if let Some(load_file) = args.load_file.clone() {
            if load_file.extension()
                    .map(|x| x.to_str()).flatten()
                    .map(|x| x.to_lowercase())
                    != Some("celeste".to_string()) {
                args.output_file = Some(load_file);
                args.load_file = None;
            }
        }
    }
    Some(args)
}
