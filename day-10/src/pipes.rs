use std::collections::HashMap;
use std::{fs::File, io::prelude::*, io::BufReader};

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Hash, Eq, Copy, Clone)]
enum PipeType {
    I,
    Bar,
    L,
    J,
    F,
    Seven,
    S,
    Ground,
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
            '.' => Self {
                pipe_type: PipeType::Ground,
                directions: (Direction::North, Direction::North),
            },
            _ => {
                panic!("Error reading character!!")
            }
        }
    }
}

pub fn pipe(file: File) -> (i32, i32) {
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
    let mut points = HashMap::new();
    let pt1_total = next_pipe(
        Direction::South,
        s_position.0 - 1 as usize,
        s_position.1,
        &chars,
        &mut points,
    );
    let pt2 = count_contents(&points, &chars) - pt1_total - 1;
    return ((pt1_total + 1) / 2, pt2);
}

fn get_loop_total(s_position: (usize, usize), chars: &Vec<Vec<char>>, points: &mut HashMap<(usize, usize), PipeType>,) -> i32 {
    if s_position.0 > 0 {
        let above = Pipe::from(chars[s_position.0 - 1][s_position.1]);
        if above.pipe_type != PipeType::Ground {
            if above.directions.0 == Direction::South || above.directions.1 == Direction::South {
                return next_pipe(
                    Direction::South,
                    s_position.0 - 1 as usize,
                    s_position.1,
                    &chars,
                    points,
                );;
            }
        }
    }
    if s_position.0 < chars.len() - 1 {
        let below = Pipe::from(chars[s_position.0 + 1][s_position.1]);
        if below.pipe_type != PipeType::Ground {
            if below.directions.0 == Direction::North || below.directions.1 == Direction::North {
                return 0;
            }
        }
    }
    if s_position.1 > 0 {
        let left = Pipe::from(chars[s_position.0][s_position.1 - 1]);
        if left.pipe_type != PipeType::Ground {
            if left.directions.0 == Direction::East || left.directions.1 == Direction::East {
                return 0;
            }
        }
    }

    return 0;
}

fn next_pipe(
    coming_from: Direction,
    i: usize,
    j: usize,
    chars: &Vec<Vec<char>>,
    points: &mut HashMap<(usize, usize), PipeType>,
) -> i32 {
    let pipe = Pipe::from(chars[i][j]);
    points.insert((i, j), pipe.pipe_type);
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
        points,
    );
}

fn count_contents(points: &HashMap<(usize, usize), PipeType>, chars: &Vec<Vec<char>>) -> i32 {
    let mut in_bounds = false;
    let mut pipe_count = 0;
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if in_bounds {
                pipe_count += 1;
                if let Some(pipe) = points.get(&(i, j)) {
                    match *pipe {
                        PipeType::J | PipeType::Seven | PipeType::I | PipeType::S => {
                            in_bounds = false;
                        }
                        _ => {}
                    };
                }
            } else {
                if let Some(_) = points.get(&(i, j)) {
                    in_bounds = true;
                    pipe_count += 1;
                }
            }
        }
    }

    return pipe_count;
}
