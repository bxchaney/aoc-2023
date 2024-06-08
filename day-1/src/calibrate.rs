use core::panic;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

pub fn read_calibration(file: File) -> Result<i32, io::Error> {
    let reader = BufReader::new(file);

    let mut total = 0;
    for l in reader.lines() {
        match l {
            Ok(s) => {
                total += match get_characters(&s) {
                    Ok(i) => {
                        print!("{} -> {}\n", s, i);
                        i
                    }
                    Err(_) => 0,
                };
            }
            Err(why) => {
                return Err(why);
            }
        };
    }

    return Ok(total);
}

pub fn get_characters(s: &str) -> Result<i32, i32> {
    let mut vec = Vec::<i32>::new();
    for c in s.chars().into_iter() {
        if let Some(i) = char_to_int(&c) {
            vec.push(i);
        }
    }

    match vec.len() {
        0 => Err(0),
        _ => Ok(vec[0] * 10 + vec[vec.len() - 1]),
    }
}

pub fn char_to_int(c: &char) -> Option<i32> {
    if is_number(c) {
        Some(*c as i32 - '0' as i32)
    } else {
        None
    }
}

pub fn is_number(c: &char) -> bool {
    "0123456789".chars().any(|y| y == *c)
}

pub fn read_calibration_letters(file: File) -> Result<i32, io::Error> {
    let reader = BufReader::new(file);
    let re = Regex::new(r"[0-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
        .unwrap();
    let re_backwards =
        Regex::new(r"[0-9]|(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)")
            .unwrap();

    let mut total = 0;
    for l in reader.lines() {
        match l {
            Ok(s) => {
                total += match get_numbers(&s, &re, &re_backwards) {
                    Ok(i) => {
                        print!(" -> {}\n", i);
                        i
                    }
                    Err(_) => 0,
                };
            }
            Err(why) => {
                return Err(why);
            }
        };
    }

    return Ok(total);
}

fn print_vec(vec: Vec<&str>) {
    print!("[");
    for s in vec {
        print!("{}, ", s);
    }
    print!("]");
}

pub fn get_numbers(s: &str, re: &Regex, re_backwards: &Regex) -> Result<i32, i32> {
    let results: Vec<&str> = re.find_iter(s).map(|c| c.as_str()).collect();
    print!("{} -> ", s);
    print_vec(results.clone());
    let result;
    if let Some(s) = re.find(s) {
        result = s.as_str();
    } else {
        return Err(-1);
    }

    let reverse_chars = s.chars().rev().collect::<String>();
    let reverse_result;
    if let Some(s) = re_backwards.find(&reverse_chars) {
        reverse_result = s.as_str().chars().rev().collect::<String>();
    } else {
        return Err(-1);
    }

    let tens = match str_to_int(result) {
        Ok(i) => i * 10,
        Err(_) => panic!("error parsing"),
    };
    let ones = match str_to_int(&reverse_result) {
        Ok(i) => i,
        Err(_) => panic!("error parsing"),
    };
    Ok(tens + ones)
}

pub fn str_to_int(s: &str) -> Result<i32, i32> {
    if s.len() == 1 {
        let vec: Vec<_> = s.chars().collect();
        if let Some(i) = char_to_int(&vec[0]) {
            Ok(i)
        } else {
            Err(-1)
        }
    } else {
        let num = match s.to_string().as_str() {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None,
        };
        if let Some(i) = num {
            Ok(i)
        } else {
            Err(-1)
        }
    }
}
