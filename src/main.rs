use std::fs;
use std::io::Read;
use std::path::Path;

mod indenter;
use indenter::indent;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(index = 1, default_value = "-")]
    path: String,

    #[arg(short, long, default_value_t = 2u8)]
    indent: u8,
}

fn main() {
    let args = Args::parse();
    let input = if args.path == "-" {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        input
    } else {
        match fs::read_to_string(Path::new(&args.path)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read file `{}`: {}", args.path, e);
                std::process::exit(1);
            }
        }
    };

    println!("{}", indent(&input, args.indent));
}
