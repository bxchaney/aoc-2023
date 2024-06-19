use std::{fs::read_to_string, iter::zip, path::Path, vec};

#[derive(Debug)]
struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
    pattern: String,
}

impl Pattern {
    fn from(pattern: String) -> Self {
        let pattern_rows = pattern.split('\n').filter(|x| x != &"");
        let mut rows = vec![];
        let mut cols = vec![];

        let mut temp: Vec<Vec<char>> = vec![];

        for row in pattern_rows {
            for (i, c) in row.chars().enumerate() {
                if temp.len() <= i {
                    let mut new_vec = vec![];
                    new_vec.push(c);
                    temp.push(new_vec);
                } else {
                    temp[i].push(c);
                }
            }
            rows.push(Self::get_encoding(row.chars().collect()));
        }

        for col_temp in temp {
            cols.push(Self::get_encoding(col_temp));
        }

        return Self {
            rows,
            cols,
            pattern,
        };
    }

    fn get_encoding(chars: Vec<char>) -> u32 {
        let mut num = 0;
        for c in chars {
            num <<= 1;
            num += if c == '#' { 1 } else { 0 };
        }
        return num;
    }

    pub fn get_total(&self, comparator: &Box<dyn Fn(&u32, &u32) -> bool>) -> u32 {
        if let Some(index) = Self::reflection_index(&self.rows, comparator) {
            return (index as u32 + 1) * 100;
        }
        if let Some(index) = Self::reflection_index(&self.cols, comparator) {
            return (index as u32) + 1;
        }

        return 0;
    }

    pub fn get_smudge_total(
        &self,
        smudge_comparator: &Box<dyn Fn(&u32, &u32, bool) -> (bool, bool)>,
    ) -> u32 {
        let default_comparator: Box<dyn Fn(&u32, &u32) -> bool> = Box::new(comp);
        let matches = (
            Self::reflection_index(&self.rows, &default_comparator),
            Self::reflection_index(&self.cols, &default_comparator),
        );
        if let Some(index) = Self::smudge_reflection_index(&self.rows, smudge_comparator, matches.0)
        {
            return (index as u32 + 1) * 100;
        }
        if let Some(index) = Self::smudge_reflection_index(&self.cols, smudge_comparator, matches.1)
        {
            return (index as u32) + 1;
        }

        return 0;
    }

    /// Returns the index after which the reflection occurs, if there is a
    /// reflection.
    fn reflection_index(
        encoding: &Vec<u32>,
        comparator: &Box<dyn Fn(&u32, &u32) -> bool>,
    ) -> Option<usize> {
        for i in 0..(encoding.len() - 1) {
            if zip(
                encoding[0..(i + 1)].iter().rev(),
                encoding[(i + 1)..].iter(),
            )
            .all(|(x, y)| comparator.as_ref()(x, y))
            {
                // true when there is a horizontal reflection after index i
                return Some(i);
            }
        }

        return None;
    }

    /// Returns the index after which the reflection occurs, if there is a
    /// reflection.
    fn smudge_reflection_index(
        encoding: &Vec<u32>,
        comparator: &Box<dyn Fn(&u32, &u32, bool) -> (bool, bool)>,
        ignored_index: Option<usize>,
    ) -> Option<usize> {
        for i in 0..(encoding.len() - 1) {
            if zip(
                encoding[0..(i + 1)].iter().rev(),
                encoding[(i + 1)..].iter(),
            )
            .fold((true, false), |acc, x| {
                let (match_found, smudge_found) = comparator.as_ref()(x.0, x.1, acc.1);
                return (acc.0 && match_found, smudge_found);
            })
            .0
            {
                if let Some(ignored) = ignored_index {
                    if ignored == i {
                        continue;
                    }
                }
                // true when there is a horizontal reflection after index i
                return Some(i);
            }
        }

        return None;
    }
}

pub fn reflection(path: &Path) -> (u32, u32) {
    let patterns: Vec<Pattern> = read_to_string(path)
        .expect("error reading file!")
        .split("\n\n")
        .map(|x| Pattern::from(x.to_string()))
        .collect();

    let pt1_comparator: Box<dyn Fn(&u32, &u32) -> bool> = Box::new(comp);
    let pt2_comparator: Box<dyn Fn(&u32, &u32, bool) -> (bool, bool)> = Box::new(smudge_comp);

    let pt1 = patterns
        .iter()
        .map(|x| x.get_total(&pt1_comparator))
        .fold(0, |acc, e| acc + e);

    let pt2 = patterns
        .iter()
        .map(|x| x.get_smudge_total(&pt2_comparator))
        .fold(0, |acc, e| acc + e);

    return (pt1, pt2);
}

fn comp(x: &u32, y: &u32) -> bool {
    x == y
}

fn smudge_comp(x: &u32, y: &u32, smudge_found: bool) -> (bool, bool) {
    if x == y {
        return (true, smudge_found);
    }
    if smudge_found {
        return (false, smudge_found);
    }

    let xor = x ^ y;
    if xor == 1 || (xor & (xor - 1) == 0) {
        return (true, true);
    }

    (false, smudge_found)
}
