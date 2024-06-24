use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct Cube {
    index: i32,
    stones: i32,
}

struct Column {
    contents: Vec<Cube>,
    length: usize,
}

impl Column {
    fn from(chars: &Vec<char>) -> Self {
        let mut contents = vec![];
        let mut current_cube = Cube {
            index: -1,
            stones: 0,
        };
        for (i, c) in chars.iter().enumerate() {
            if *c == 'O' {
                current_cube.stones += 1;
            } else if *c == '#' {
                contents.push(current_cube);
                current_cube = Cube {
                    index: i as i32,
                    stones: 0,
                }
            }
        }
        contents.push(current_cube);

        Self {
            contents,
            length: chars.len(),
        }
    }

    fn calc_load(&self) -> i32 {
        self.contents
            .iter()
            .map(|x| {
                Self::pyramid_sum(self.length as i32 - x.index - 1)
                    - Self::pyramid_sum(self.length as i32 - x.index - 1 - x.stones)
            })
            .fold(0, |acc, x| acc + x)
    }

    fn pyramid_sum(i: i32) -> i32 {
        (i * (i + 1)) / 2
    }
}

pub fn tilt(path: &Path) -> (i32, i64) {
    let file = File::open(path).expect("Error opening file!");
    let reader = BufReader::new(file);

    let mut panel: Vec<Vec<char>> = vec![];

    for l in reader.lines().flatten() {
        for (i, c) in l.chars().enumerate() {
            if panel.len() <= i {
                let new_col = vec![];
                panel.push(new_col);
            }
            panel[i].push(c);
        }
    }

    let pt1 = panel
        .iter()
        .map(|x| Column::from(x).calc_load())
        .fold(0, |acc, x| acc + x);

    return (pt1, 0);
}
