use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, io};

struct Card {
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Card {
    fn from(line: &str) -> Self {
        let numbers_str: &str;
        let winning_numbers;
        let numbers;
        if let Some((_, number_list_str)) = line.split_once(':') {
            numbers_str = number_list_str;
        } else {
            panic!("Line does not have expected format!!");
        }

        if let Some((winning_number_str, number_str)) = numbers_str.split_once('|') {
            winning_numbers = winning_number_str
                .split(' ')
                .filter(|x| x.trim() != "")
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();
            numbers = number_str
                .split(' ')
                .filter(|x| x.trim() != "")
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();
        } else {
            panic!("Line does not have expected format!!");
        }
        Self {
            winning_numbers,
            numbers,
        }
    }
}

pub fn read_input(file: File, number: i32) -> Result<i32, io::Error> {
    let reader = BufReader::new(file);
    let mut vec = vec![];

    let mut total = 0;
    for l in reader.lines() {
        match l {
            Ok(s) => {
                let card = Card::from(&s);
                vec.push(card);
            }
            Err(why) => {
                return Err(why);
            }
        }
    }
    if number == 1 {
        for card in vec {
            let intersection = card.winning_numbers.intersection(&card.numbers).count();
            total += if intersection == 0 {
                0
            } else {
                2_i32.pow(intersection as u32 - 1)
            };
        }
    } else {
        total = count_copies(&vec);
    }
    return Ok(total);
}

fn count_copies(vec: &Vec<Card>) -> i32 {
    let mut memo: Vec<Option<i32>> = vec![Option::None; vec.len()];
    let mut total = 0;
    for i in 0..vec.len() {
        total += copies(i, vec, &mut memo);
    }

    return total;
}

fn copies(i: usize, vec: &Vec<Card>, memo: &mut Vec<Option<i32>>) -> i32 {
    if let Some(num) = memo[i] {
        return num;
    }
    let intersection_size = vec[i].winning_numbers.intersection(&vec[i].numbers).count();
    let mut total = 1;
    for n in 1..intersection_size + 1 {
        total += copies(i + n, vec, memo);
    }
    memo[i] = Some(total);
    return total;
}
