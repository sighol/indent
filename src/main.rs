use std::io::Read;

mod indenter;
use indenter::indent;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", indent(&input));
}
