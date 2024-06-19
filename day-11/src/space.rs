use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[derive(Debug, Copy, Clone)]
struct Galaxy {
    x: usize,
    y: usize,
}

pub fn space(file: File) -> (i64, i64) {
    let reader = BufReader::new(file);
    let mut galaxies_per_row: Vec<(usize, Option<i32>)> = vec![];
    let mut galaxies_per_column: Vec<(usize, Option<i32>)> = vec![];
    let mut galaxies: Vec<Galaxy> = vec![];

    for (i, l) in reader
        .lines()
        .map(|x| x.expect("problem opening reading file!"))
        .enumerate()
    {
        galaxies_per_row.push((i, None));
        for (j, c) in l.chars().enumerate() {
            if galaxies_per_column.len() <= j {
                galaxies_per_column.push((j, None));
            }
            if c == '.' {
                // not galaxy, ignore
                continue;
            }

            // otherwise, record galaxy
            if let Some(row_count) = galaxies_per_row[i].1 {
                galaxies_per_row[i] = (i, Some(row_count + 1));
            } else {
                galaxies_per_row[i] = (i, Some(1));
            }

            if let Some(col_count) = galaxies_per_column[j].1 {
                galaxies_per_column[j] = (j, Some(col_count + 1));
            } else {
                galaxies_per_column[j] = (j, Some(1));
            }

            galaxies.push(Galaxy { x: j, y: i });
        }
    }

    let mut gal_copy = galaxies.clone();

    expand_galaxies(
        &mut galaxies,
        galaxies_per_row.clone(),
        galaxies_per_column.clone(),
        2,
    );

    for gal in &gal_copy {
        println!("{:?}", gal);
    }
    expand_galaxies(
        &mut gal_copy,
        galaxies_per_row,
        galaxies_per_column,
        1_000_000,
    );

    for gal in &gal_copy {
        println!("{:?}", gal);
    }

    return (distance_sum(&galaxies), distance_sum(&gal_copy));
}

fn expand_galaxies(
    galaxies: &mut Vec<Galaxy>,
    galaxies_per_row: Vec<(usize, Option<i32>)>,
    galaxies_per_col: Vec<(usize, Option<i32>)>,
    expansion_factor: usize,
) {
    for row in galaxies_per_row.iter().rev() {
        if let Some(_) = row.1 {
            // there are galaxies here, no expansion!
            continue;
        } else {
            for gal in &mut *galaxies {
                if gal.y >= row.0 {
                    gal.y += expansion_factor - 1;
                }
            }
        }
    }

    for col in galaxies_per_col.iter().rev() {
        if let Some(_) = col.1 {
            continue;
        } else {
            for gal in &mut *galaxies {
                if gal.x >= col.0 {
                    gal.x += expansion_factor - 1;
                }
            }
        }
    }
}

fn distance_sum(galaxies: &Vec<Galaxy>) -> i64 {
    let mut total = 0;
    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            total += (galaxies[i].x as i64 - galaxies[j].x as i64).abs()
                + (galaxies[i].y as i64 - galaxies[j].y as i64).abs();
        }
    }
    return total;
}
