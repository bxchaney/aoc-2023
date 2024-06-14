use std::{fs::File, io::prelude::*, io::BufReader};

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq)]
enum PipeType {
    I,
    Bar,
    L,
    J,
    F,
    Seven,
    S,
}

struct Pipe {
    pipe_type: PipeType,
    directions: (Direction, Direction),
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn offset(&self) -> (i32, i32) {
        match *self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }
}

impl Pipe {
    pub fn from(c: char) -> Self {
        match c {
            '|' => Self {
                pipe_type: PipeType::I,
                directions: (Direction::North, Direction::South),
            },
            '-' => Self {
                pipe_type: PipeType::Bar,
                directions: (Direction::East, Direction::West),
            },
            'L' => Self {
                pipe_type: PipeType::L,
                directions: (Direction::North, Direction::East),
            },
            'J' => Self {
                pipe_type: PipeType::J,
                directions: (Direction::North, Direction::West),
            },
            '7' => Self {
                pipe_type: PipeType::Seven,
                directions: (Direction::West, Direction::South),
            },
            'F' => Self {
                pipe_type: PipeType::F,
                directions: (Direction::East, Direction::South),
            },
            'S' => Self {
                pipe_type: PipeType::S,
                directions: (Direction::North, Direction::West),
            },
            _ => {
                panic!("Error reading character!!")
            }
        }
    }
}

pub fn pipe(file: File) -> (i32,) {
    let reader = BufReader::new(file);
    let mut chars: Vec<Vec<char>> = vec![];
    let mut s_position = (0, 0);
    let mut row_count = 0;
    for l in reader
        .lines()
        .map(|x| x.expect("problem opening reading file!"))
    {
        let row: Vec<char> = l.chars().collect();
        if let Some(i) = row.iter().position(|&r| r == 'S') {
            s_position = (row_count, i);
        }
        chars.push(row);
        row_count += 1;
    }
    let pt1_total = next_pipe(
        Direction::South,
        s_position.0 - 1 as usize,
        s_position.1,
        &chars,
    );
    return ((pt1_total + 1) / 2,);
}

fn next_pipe(coming_from: Direction, i: usize, j: usize, chars: &Vec<Vec<char>>) -> i32 {
    let pipe = Pipe::from(chars[i][j]);
    if pipe.pipe_type == PipeType::S {
        return 0;
    }
    let going_to;
    if pipe.directions.0 == coming_from {
        going_to = pipe.directions.1;
    } else {
        going_to = pipe.directions.0;
    }
    let offset = going_to.offset();
    return 1 + next_pipe(
        going_to.opposite(),
        ((i as i32) + offset.0) as usize,
        ((j as i32) + offset.1) as usize,
        chars,
    );
}
