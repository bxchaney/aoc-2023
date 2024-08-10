use std::{env, path::Path};

use pulse::pulse;

mod pulse;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            panic!("not enough args!");
        }
        2 => {}
        _ => {
            panic!("too many args!");
        }
    };

    let filepath = &args[1];
    let path = Path::new(filepath);

    let (pt1, pt2) = pulse(&path);

    println!("pt1: {:?}", pt1.unwrap());
    println!("pt2: {:?}", pt2.unwrap());
}
