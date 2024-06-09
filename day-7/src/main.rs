use std::{env, fs::File, path::Path};
mod card;

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
    let file = File::open(&path).expect("error opening file!");

    let part_one = card::part_one(file);
    println!("pt1: {}", part_one)
}
