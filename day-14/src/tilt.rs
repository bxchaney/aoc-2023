use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Debris {
    Stone,
    Cube,
    Blank,
}

enum ShiftType {
    Up,
    Down,
    Left,
    Right,
}

struct Panel {
    panel: Vec<Vec<Debris>>,
    row_rocks: Vec<Vec<usize>>,
    col_rocks: Vec<Vec<usize>>,
}

impl Panel {
    fn new() -> Self {
        Self {
            panel: vec![],
            row_rocks: vec![],
            col_rocks: vec![],
        }
    }

    fn add_row(&mut self, row: String) {
        let mut new_row = vec![];
        self.row_rocks.push(vec![]);
        for (i, c) in row.chars().enumerate() {
            if self.col_rocks.len() <= i {
                self.col_rocks.push(vec![]);
            }
            let debris = match c {
                '#' => Debris::Cube,
                'O' => Debris::Stone,
                _ => Debris::Blank,
            };
            if debris == Debris::Cube {
                self.col_rocks[i].push(self.panel.len());
                self.row_rocks[self.panel.len()].push(i);
            }
            new_row.push(debris);
        }
        self.panel.push(new_row);
    }

    fn shift(&mut self, shift_type: ShiftType) {
        match shift_type {
            ShiftType::Up => {
                self.vertical_shift(true);
            }
            ShiftType::Down => {
                self.vertical_shift(false);
            }
            ShiftType::Left => {
                self.horizontal_shift(true);
            }
            ShiftType::Right => {
                self.horizontal_shift(false);
            }
        }
    }

    fn vertical_shift(&mut self, north: bool) {
        let rocks = self.col_rocks.clone();
        for (i, col_rocks) in rocks.iter().enumerate() {
            let mut ranges = vec![];
            if col_rocks.len() == 0 {
                ranges.push((0, self.panel.len()));
            } else {
                let mut start = 0;
                for j in col_rocks {
                    ranges.push((start, *j));
                    start = *j + 1;
                }
                ranges.push((start, self.panel.len()));
            }

            self.sort_vert_range(i, &ranges, north);
        }
    }

    fn sort_vert_range(&mut self, column: usize, ranges: &Vec<(usize, usize)>, north: bool) {
        for (start, stop) in ranges {
            let mut stones = 0;
            let mut blanks = 0;
            for i in *start..*stop {
                match self.panel[i][column] {
                    Debris::Stone => {
                        stones += 1;
                    }
                    Debris::Blank => {
                        blanks += 1;
                    }
                    _ => {}
                }
            }
            let primary;
            let secondary;
            let mut primary_count;
            if north {
                primary_count = stones;
                primary = Debris::Stone;
                secondary = Debris::Blank;
            } else {
                primary_count = blanks;
                primary = Debris::Blank;
                secondary = Debris::Stone;
            }
            for i in *start..*stop {
                if primary_count > 0 {
                    self.panel[i][column] = primary;
                    primary_count -= 1;
                } else {
                    self.panel[i][column] = secondary;
                }
            }
        }
    }

    fn horizontal_shift(&mut self, west: bool) {
        let rocks = self.row_rocks.clone();
        for (i, row_rocks) in rocks.iter().enumerate() {
            let mut ranges = vec![];
            if row_rocks.len() == 0 {
                ranges.push((0, self.panel[0].len()));
            } else {
                let mut start = 0;
                for j in row_rocks {
                    ranges.push((start, *j));
                    start = *j + 1;
                }
                ranges.push((start, self.panel[0].len()));
            }
            self.sort_horz_range(i, &ranges, west);
        }
    }

    fn sort_horz_range(&mut self, row: usize, ranges: &Vec<(usize, usize)>, west: bool) {
        for (start, stop) in ranges {
            let mut stones = 0;
            let mut blanks = 0;
            for j in *start..*stop {
                match self.panel[row][j] {
                    Debris::Stone => {
                        stones += 1;
                    }
                    Debris::Blank => {
                        blanks += 1;
                    }
                    _ => {}
                }
            }
            let primary;
            let secondary;
            let mut primary_count;
            if west {
                primary_count = stones;
                primary = Debris::Stone;
                secondary = Debris::Blank;
            } else {
                primary_count = blanks;
                primary = Debris::Blank;
                secondary = Debris::Stone;
            }
            for j in *start..*stop {
                if primary_count > 0 {
                    self.panel[row][j] = primary;
                    primary_count -= 1;
                } else {
                    self.panel[row][j] = secondary;
                }
            }
        }
    }

    fn get_load(&self) -> i32 {
        let mut total = 0;
        let total_rows = self.panel.len() as i32;
        for (i, row) in self.panel.iter().enumerate() {
            for d in row {
                match d {
                    Debris::Stone => {
                        total += total_rows - (i as i32);
                    }
                    _ => {}
                }
            }
        }
        return total;
    }

    pub fn cycle(&mut self) {
        self.shift(ShiftType::Up);
        self.shift(ShiftType::Left);
        self.shift(ShiftType::Down);
        self.shift(ShiftType::Right);
    }
}

pub fn tilt(path: &Path) -> (i32, i32) {
    let file = File::open(path).expect("Error opening file!");
    let reader = BufReader::new(file);

    let mut panel = Panel::new();

    for l in reader.lines().flatten() {
        panel.add_row(l);
    }

    panel.shift(ShiftType::Up);
    let pt1 = panel.get_load();

    panel.shift(ShiftType::Left);
    panel.shift(ShiftType::Down);
    panel.shift(ShiftType::Right);

    for _ in 0..999 {
        panel.cycle();
    }

    let pt2 = panel.get_load();

    return (pt1, pt2);
}
