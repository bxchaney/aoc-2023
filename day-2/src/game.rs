use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, io};

const RED_LIMIT: i32 = 12;
const GREEN_LIMIT: i32 = 13;
const BLUE_LIMIT: i32 = 14;

const RED: &'static str = "red";
const BLUE: &'static str = "blue";

pub struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

pub struct GameSet {
    red: i32,
    blue: i32,
    green: i32,
}

impl Game {
    pub fn from(game: &str) -> Self {
        let s: Vec<&str> = game.split(':').collect();
        if s.len() < 2 {
            return Self {
                id: 127,
                sets: vec![],
            };
        }

        // the game is is contained in a string with the form:
        // Game xxx, where the game id could be 1, 2, or 3 characters.
        // The first caracter of the game id starts at index 5 and the last
        // character ends at the last index of the string
        let id = s[0][5..].parse::<i32>().unwrap();
        let sets = s[1]
            .split(';')
            .map(|x| GameSet::from(x))
            .collect::<Vec<_>>();
        Self { id, sets }
    }
}

impl GameSet {
    pub fn from(set: &str) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        print!("\n\t'{}'\n", set);

        for s in set.split(',') {
            println!("\t\t'{}'", s);

            if &s[(s.len() - 3)..] == RED {
                red += s[1..(s.len() - 4)].parse::<i32>().unwrap();
            } else if &s[(s.len() - 4)..] == BLUE {
                blue += s[1..(s.len() - 5)].parse::<i32>().unwrap();
            } else {
                green += s[1..(s.len() - 6)].parse::<i32>().unwrap();
            }
        }

        return Self { red, blue, green };
    }
}

pub fn read_games(file: File) -> Result<i32, io::Error> {
    let reader = BufReader::new(file);

    let mut total = 0;
    for l in reader.lines() {
        match l {
            Ok(s) => {
                print!("{}", s);
                let game = Game::from(&s);
                if !game.sets.iter().any(|x| is_invalid_set(x)) {
                    total += game.id;
                    print!(" -> total: {}", game.id);
                }
                print!("\n");
            }
            Err(why) => {
                return Err(why);
            }
        }
    }

    return Ok(total);
}

pub fn is_invalid_set(set: &GameSet) -> bool {
    set.red > RED_LIMIT || set.blue > BLUE_LIMIT || set.green > GREEN_LIMIT
}

pub fn get_powers(file: File) -> Result<i32, io::Error> {
    let reader = BufReader::new(file);

    let mut total = 0;
    for l in reader.lines() {
        match l {
            Ok(s) => {
                print!("{}", s);
                let game = Game::from(&s);
                let min_set = get_min_set(&game.sets);
                total += min_set.red * min_set.blue * min_set.green;
                print!(" -> total: {}", game.id);
                print!("\n");
            }
            Err(why) => {
                return Err(why);
            }
        }
    }

    return Ok(total);
}

fn get_min_set(sets: &Vec<GameSet>) -> GameSet {
    let mut min_set = GameSet {
        red: 0,
        blue: 0,
        green: 0,
    };
    for set in sets {
        if set.red > min_set.red {
            min_set.red = set.red;
        }
        if set.blue > min_set.blue {
            min_set.blue = set.blue;
        }
        if set.green > min_set.green {
            min_set.green = set.green;
        }
    }
    return min_set;
}
