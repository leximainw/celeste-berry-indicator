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

fn parse_args_core(iter: &mut dyn Iterator<Item=String>) -> Option<Args> {
    let mut args = Args::new();
    for arg in iter {
        let mut chars = arg.chars();
        let next = chars.next();
        if next == Some('-') || cfg!(target_os = "windows") && next == Some('/') {
            if next == Some('/') || chars.next() == Some('-') {
                let arg = &arg.to_lowercase()[if next == Some('/') { 1 } else { 2 }..];
                let prefix = if next == Some('/') { "/" } else { "--" };
                if arg.starts_with("bg=") {
                    args.background = arg[3..].to_string();
                } else if arg == "help" {
                    println!("{prefix}bg={{bg}} - - - - render the image with a specified background");
                    println!("{prefix}help  - - - - - print this list");
                    println!("{prefix}help:bg - - - - print a list of available backgrounds");
                    println!("{prefix}deaths  - - - - render death count");
                    println!("{prefix}id={{0..2}} - - - select save file by ID");
                    println!("{prefix}no-hearts - - - don't render hearts");
                    println!("{prefix}no-spoilers - - hide uncollected items from incomplete levels");
                    println!("{prefix}output={{file}} - write to specified file");
                    println!("{prefix}spacing - - - - add space between adjacent hearts");
                    return None;
                } else if arg.starts_with("help:") {
                    let category = &arg[5..];
                    match category {
                        "bg" => {
                            println!("{prefix}bg=trans  - - transgender flag (default)");
                            println!("{prefix}bg=aroace - - aroace flag");
                            println!("{prefix}bg=aro  - - - aromantic flag");
                            println!("{prefix}bg=ace  - - - asexual flag");
                            println!("{prefix}bg=bi - - - - bisexual flag");
                            println!("{prefix}bg=enby - - - nonbinary flag");
                            println!("{prefix}bg=gay  - - - gay (men loving men) flag");
                            println!("{prefix}bg=lesbian  - lesbian (women loving women) flag");
                            println!("{prefix}bg=lesbian5 - lesbian flag (5-stripe version)");
                            println!("{prefix}bg=queer  - - queer flag");
                            println!("{prefix}bg=intersex - intersex flag");
                            println!("{prefix}bg=rainbow  - rainbow flag (6-stripe version)");
                            println!("{prefix}bg=old-progress - progress pride flag");
                            println!("{prefix}bg=progress - intersex progress pride flag");
                        },
                        _ => println!("unknown category {category}"),
                    }
                    return None;
                } else if arg == "deaths" {
                    args.show_deaths = true;
                } else if arg.starts_with("id=") {
                    if let Ok(id) = str::parse::<usize>(&arg[3..]) {
                        args.load_id = Some(id);
                    }
                } else if arg == "no-hearts" {
                    args.show_hearts = false;
                } else if arg == "no-spoilers" {
                    args.hide_incomplete = true;
                } else if arg.starts_with("output=") {
                    let mut output_file: PathBuf = arg[7..].into();
                    if output_file.extension() == None {
                        output_file.set_extension(DEFAULT_EXTENSION);
                    }
                    args.output_file = Some(output_file);
                } else if arg == "spacing" {
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
