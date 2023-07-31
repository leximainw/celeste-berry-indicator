use std::path::PathBuf;

pub struct Args {
    pub load_file: Option<PathBuf>,
    pub load_id: Option<usize>,
    pub show_deaths: bool,
    pub show_hearts: bool,
}

impl Args {
    pub fn new() -> Args {
        Args{
            load_file: None,
            load_id: None,
            show_deaths: false,
            show_hearts: true,
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
                if arg == "--deaths" {
                    args.show_deaths = true;
                } else if arg == "--hearts" {
                    args.show_hearts = true;
                } else if arg.starts_with("--id=") {
                    if let Ok(id) = str::parse::<usize>(&arg[5..]) {
                        args.load_id = Some(id);
                    }
                }
            }
        } else if args.load_file == None {
            args.load_file = Some(arg.into());
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
            if arg == "/deaths" {
                args.show_deaths = true;
            } else if arg == "/hearts" {
                args.show_hearts = true;
            } else if arg.starts_with("/id=") {
                if let Ok(id) = str::parse::<usize>(&arg[4..]) {
                    args.load_id = Some(id);
                }
            }
        } else if args.load_file == None {
            args.load_file = Some(arg.into());
        }
    }
    args
}