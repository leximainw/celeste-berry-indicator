use std::path::PathBuf;

const DEFAULT_EXTENSION: &str = "bmp";

pub struct Args {
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

pub fn parse_args() -> Args {
    parse_args_core(&mut std::env::args().skip(1))
}

#[cfg(not(target_os = "windows"))]
fn parse_args_core(iter: &mut dyn Iterator<Item=String>) -> Args {
    let mut args = Args::new();
    for arg in iter {
        let mut chars = arg.chars();
        if chars.next() == Some('-') {
            if chars.next() == Some('-') {
                let arg = arg.to_lowercase();
                if arg == "--help" {
                    println!("--help  - - - - - print this list");
                    println!("--deaths  - - - - render death count");
                    println!("--id={{0..2}} - - - select save file by ID");
                    println!("--no-hearts - - - don't render hearts");
                    println!("--no-spoilers - - hide uncollected items from incomplete levels");
                    println!("--output={{file}} - write to specified file");
                    println!("--spacing - - - - add space between adjacent hearts");
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
    args
}

#[cfg(target_os = "windows")]
fn parse_args_core(iter: &mut dyn Iterator<Item=String>) -> Args {
    let mut args = Args::new();
    for arg in iter {
        let mut chars = arg.chars();
        if chars.next() == Some('/') {
            let arg = arg.to_lowercase();
            if arg == "/help" {
                println!("/help  - - - - - print this list");
                println!("/deaths  - - - - render death count");
                println!("/id={{0..2}} - - - select save file by ID");
                println!("/no-hearts - - - render hearts if golden_count == 0 || !heart");
                println!("/no-spoilers - - hide uncollected items from incomplete levels");
                println!("/output={{file}} - write to specified file");
                println!("/spacing - - - - add space between adjacent hearts");
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
    args
}
