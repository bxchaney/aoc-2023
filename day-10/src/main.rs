use std::{env, fs::File, path::Path};

mod pipes;
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

    let pt1 = pipes::pipe(file);

    println!("pt1: {}", pt1.0);
}
