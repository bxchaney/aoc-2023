use std::{env, path::Path};

mod reflection;
use reflection::reflection;

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

    let (pt1, pt2) = reflection(&path);

    println!("pt1: {}", pt1);
    println!("pt2: {}", pt2);
}
