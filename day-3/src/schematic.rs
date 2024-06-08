use std::cmp::min;
use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, io};

#[derive(Copy, Clone)]
struct SchematicNumber {
    row: usize,
    col: usize,
    length: usize,
    value: i64,
}

#[derive(Clone, Copy)]
pub struct Symbol {
    character: char,
}

pub struct Schematic {
    numbers: Vec<SchematicNumber>,
    symbols: Vec<Vec<Option<Symbol>>>,
}

#[derive(Clone, Copy)]
pub struct Part {
    symbol: Symbol,
    part_count: i32,
    total: i64,
}

impl Symbol {
    pub fn new(character: char) -> Self {
        Self { character }
    }
}

impl SchematicNumber {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            length: 0,
            value: 0,
        }
    }
}

impl Schematic {
    pub fn new() -> Self {
        return Self {
            numbers: vec![],
            symbols: vec![vec![Option::<Symbol>::None; 140]; 140],
        };
    }

    pub fn process_row(&mut self, row_number: usize, row: &str) {
        let mut i = 0;
        let row_chars: Vec<char> = row.chars().collect();
        while i < row_chars.len() {
            if self.is_symbolic(&row_chars[i]) {
                self.symbols[row_number][i] = Some(Symbol::new(row_chars[i]));
                i += 1;
            } else if self.is_numeric(&row_chars[i]) {
                // if we find a number, store its location and value
                let mut j = 0;
                let mut number = SchematicNumber::new(row_number, i);
                while i + j < row_chars.len() && self.is_numeric(&row_chars[i + j]) {
                    number.value *= 10;
                    number.value += row_chars[i + j] as i64 - '0' as i64;
                    number.length += 1;
                    j += 1;
                }
                self.numbers.push(number);
                i += j;
            } else {
                i += 1;
            }
        }
    }

    fn is_numeric(&self, c: &char) -> bool {
        "0123456789".chars().any(|y| y == *c)
    }

    fn is_symbolic(&self, c: &char) -> bool {
        !self.is_numeric(c) && !(*c == '.')
    }
}

impl Part {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            symbol,
            part_count: 0,
            total: 1,
        }
    }
}

pub fn read_input(file: File, number: i32) -> Result<i64, io::Error> {
    let mut schematic = Schematic::new();
    let reader = BufReader::new(file);

    for (i, l) in reader.lines().enumerate() {
        match l {
            Ok(s) => {
                schematic.process_row(i, &s);
            }
            Err(why) => {
                return Err(why);
            }
        }
    }
    if number == 1 {
        return Ok(get_total(schematic));
    } else {
        return Ok(get_gear_ratio(schematic));
    }
}

fn get_total(schematic: Schematic) -> i64 {
    let mut total = 0;
    for number in schematic.numbers {
        let part_number;
        // no corner numbers in my dataset hehe
        if number.col == 0 {
            // check row above
            // check row below
            // check tile to right
            part_number = exists_adjacent_symbol(
                &schematic.symbols[number.row - 1][number.col..number.col + number.length + 1],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row]
                    [number.col + number.length..number.col + number.length + 2],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row + 1][number.col..number.col + number.length + 1],
            );
        } else if number.col + number.length >= 139 {
            // check row above
            // check row below
            // check tile to left
            part_number = exists_adjacent_symbol(
                &schematic.symbols[number.row - 1][number.col - 1..number.col + number.length],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row][number.col - 1..number.col],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row + 1][number.col - 1..number.col + number.length],
            );
        } else if number.row == 0 {
            // check tile to left
            // check tile to right
            // check row below
            part_number = exists_adjacent_symbol(
                &schematic.symbols[number.row]
                    [number.col + number.length..number.col + number.length + 1],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row][number.col - 1..number.col],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row + 1][number.col - 1..number.col + number.length + 1],
            );
        } else if number.row >= 139 {
            // check tile to left
            // check tile to right
            // check row above
            part_number = exists_adjacent_symbol(
                &schematic.symbols[number.row - 1][number.col - 1..number.col + number.length + 1],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row]
                    [number.col + number.length..number.col + number.length + 1],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row][number.col - 1..number.col],
            );
        } else {
            // check row above
            // check tile to left
            // check tile to right
            // check row below
            part_number = exists_adjacent_symbol(
                &schematic.symbols[number.row - 1][number.col - 1..number.col + number.length + 1],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row]
                    [number.col + number.length..number.col + number.length + 1],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row][number.col - 1..number.col],
            ) || exists_adjacent_symbol(
                &schematic.symbols[number.row + 1][number.col - 1..number.col + number.length + 1],
            );
        }
        if part_number {
            total += number.value;
        }
    }

    return total;
}

fn exists_adjacent_symbol(vec: &[Option<Symbol>]) -> bool {
    vec.iter().any(|x| match x {
        Some(_) => true,
        None => false,
    })
}
fn find_adjacent_symbol<'a>(
    parts: &mut Vec<Option<Part>>,
    number: SchematicNumber,
    schematic_symbols: &Vec<Vec<Option<Symbol>>>,
) {
    let starting_col = if number.col == 0 { 0 } else { number.col - 1 };
    let ending_col = min(140, number.col + number.length + 1);
    // check above
    if number.row > 0 {
        for i in starting_col..ending_col {
            if let Some(symbol) = schematic_symbols[number.row - 1][i] {
                add_part_number(parts, number.row - 1, i, number, symbol);
            }
        }
    }
    // check below
    if number.row < 139 {
        for i in starting_col..ending_col {
            if let Some(symbol) = schematic_symbols[number.row + 1][i] {
                add_part_number(parts, number.row + 1, i, number, symbol);
            }
        }
    }

    // check left
    if number.col > 0 {
        if let Some(symbol) = schematic_symbols[number.row][number.col - 1] {
            add_part_number(parts, number.row, number.col - 1, number, symbol);
        }
    }
    // check right
    if number.col + number.length < 140 {
        if let Some(symbol) = schematic_symbols[number.row][number.col + number.length] {
            add_part_number(
                parts,
                number.row,
                number.col + number.length,
                number,
                symbol,
            );
        }
    }
}

fn add_part_number<'a>(
    parts: &mut Vec<Option<Part>>,
    row: usize,
    col: usize,
    number: SchematicNumber,
    symbol: Symbol,
) {
    let index = row * 140 + col;
    let mut updated_part;
    if let Some(part) = parts[index] {
        updated_part = part.clone();
    } else {
        updated_part = Part::new(symbol);
    }
    updated_part.part_count += 1;
    updated_part.total *= number.value;
    parts[index] = Some(updated_part);
}

fn get_gear_ratio(schematic: Schematic) -> i64 {
    let mut parts: Vec<Option<Part>> = vec![Option::None; 140 * 140];
    for number in schematic.numbers {
        find_adjacent_symbol(&mut parts, number, &schematic.symbols);
    }
    let total = parts
        .iter()
        .filter(|x| match x {
            Some(p) => p.part_count == 2 && p.symbol.character == '*',
            None => false,
        })
        .map(|x| match x {
            Some(p) => p.total,
            None => 0,
        })
        .reduce(|acc, e| acc + e)
        .unwrap();
    return total;
}
