use core::panic;
use std::{env, fs::File, path::Path};

mod calibrate;

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
    let path = Path::new(filepath);
    let total;
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(file) => file,
    };

    if number == 1 {
        total = calibrate::read_calibration(file);
    } else {
        total = calibrate::read_calibration_letters(file);
    }
    match total {
        Ok(i) => print!("total is: {}\n", i),
        Err(why) => eprint!("error: {}\n", why),
    };
}

#[test]
fn validate_is_number() {
    assert!(calibrate::is_number(&'0'));
    assert!(calibrate::is_number(&'1'));
    assert!(calibrate::is_number(&'2'));
    assert!(calibrate::is_number(&'3'));
    assert!(calibrate::is_number(&'4'));
    assert!(calibrate::is_number(&'5'));
    assert!(calibrate::is_number(&'6'));
    assert!(calibrate::is_number(&'7'));
    assert!(calibrate::is_number(&'8'));
    assert!(calibrate::is_number(&'9'));
    assert!(!calibrate::is_number(&'a'));
}

#[test]
fn valiadate_get_characters() {
    assert_eq!(calibrate::get_characters(&"asdf123"), Ok(13));
    assert_eq!(calibrate::get_characters(&"asdf123asdf"), Ok(13));
    assert_eq!(calibrate::get_characters(&"asdf1asdf"), Ok(11));
    assert_eq!(calibrate::get_characters(&"123"), Ok(13));
}

#[test]
fn test_char_to_int() {
    assert_eq!(calibrate::char_to_int(&'0'), Some(0));
    assert_eq!(calibrate::char_to_int(&'4'), Some(4));
    assert_eq!(calibrate::char_to_int(&'6'), Some(6));
    assert_eq!(calibrate::char_to_int(&'7'), Some(7));
    assert_eq!(calibrate::char_to_int(&'a'), None);
}

#[test]
fn test_regex() {
    let re = Regex::new(r"[0-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
        .unwrap();
    
}
