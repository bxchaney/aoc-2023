use core::panic;
use std::{env, fs::File, path::Path};

mod schematic;

fn main() {
    let args: Vec<String> = env::args().collect();

    let number: i32;

    match args.len() {
        1 | 2 => {
            panic!("not enough arguments!");
        }

        3 => {
            let flag = &args[1];
            number = match flag.parse() {
                Ok(1) => 1,
                Ok(2) => 2,
                Err(_) => {
                    eprintln!("error, flag is not an integer");
                    return;
                }
                Ok(_) => {
                    eprintln!("error, only supports 1 or 2 as the second flags");
                    return;
                }
            };
        }

        _ => {
            eprintln!("error, too man args");
            panic!();
        }
    };

    let filepath = &args[2];
    let total;
    let path = Path::new(filepath);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(file) => file,
    };

    match schematic::read_input(file, number) {
        Ok(x) => {
            total = x;
        }
        Err(why) => panic!("error: {}", why),
    };

    println!("total: {}", total);
}
