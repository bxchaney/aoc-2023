use std::{env, fs::File, path::Path};

mod coords;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 | 2 => {
            panic!("not enough args!");
        }
        3 => {}
        _ => {
            panic!("too many args!");
        }
    };

    let filepath = &args[2];
    let path = Path::new(filepath);
    let map = File::open(&path).expect("error opening file!");

    let dir_filepath = &args[1];
    let dir_path = Path::new(dir_filepath);
    let directions = File::open(&dir_path).expect("problem opening directions file!");

    // println!(
    //     "FSA: {}",
    //     coords::part_one(String::from("FSA"), directions, map)
    // );
    // println!(
    //     "JVA: {}",
    //     coords::part_one(String::from("JVA"), directions, map)
    // );
    println!("pt2: {}", coords::part_two(directions, map));
    // println!(
    //     "KNA: {}",
    //     coords::part_one(String::from("KNA"), directions, map)
    // );
    // println!(
    //     "AAA: {}",
    //     coords::part_one(String::from("AAA"), directions, map)
    // );
    // println!(
    //     "FXA: {}",
    //     coords::part_one(String::from("FXA"), directions, map)
    // );
}
