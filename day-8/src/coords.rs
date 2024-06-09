use std::collections::HashMap;
use std::{fs::File, io::prelude::*, io::BufReader};

struct Children {
    left: String,
    right: String,
}

pub fn part_one(starting_str: String, directions: File, map: File) -> i32 {
    let mut dir_reader = BufReader::new(directions);
    let mut dir: String = String::new();
    dir_reader
        .read_line(&mut dir)
        .expect("Problem reading directions file");
    let mut dict: HashMap<String, Children> = HashMap::new();
    let map_reader = BufReader::new(map);
    for l in map_reader
        .lines()
        .map(|x| x.expect("Error reading line in map file!"))
    {
        dict.insert(
            l[0..3].to_string(),
            Children {
                left: l[7..10].to_string(),
                right: l[12..15].to_string(),
            },
        );
    }

    let mut current_location = starting_str;
    let mut total = 0;
    'outer: loop {
        for c in dir.to_string().chars() {
            let new_location = match c {
                'L' => &dict.get(&current_location).unwrap().left,
                'R' => &dict.get(&current_location).unwrap().right,
                _ => panic!("invalid direction!"),
            };
            total += 1;
            current_location = new_location.to_string();
            if current_location.chars().nth(2) == Some('Z') {
                break 'outer;
            }
        }
    }

    return total;
}

fn steps_to_z(starting_str: String, dict: &HashMap<String, Children>, directions: String) -> i32 {
    let mut current_location = starting_str;
    let mut total = 0;
    'outer: loop {
        for c in directions.chars() {
            let new_location = match c {
                'L' => &dict.get(&current_location).unwrap().left,
                'R' => &dict.get(&current_location).unwrap().right,
                _ => panic!("invalid direction!"),
            };
            total += 1;
            current_location = new_location.to_string();
            if current_location.chars().nth(2) == Some('Z') {
                break 'outer;
            }
        }
    }

    return total;
}

pub fn part_two(directions: File, map: File) -> u64 {
    let mut dir_reader = BufReader::new(directions);
    let mut dir: String = String::new();
    dir_reader
        .read_line(&mut dir)
        .expect("Problem reading directions file");
    let mut dict: HashMap<String, Children> = HashMap::new();
    let mut current_nodes: Vec<String> = vec![];
    let map_reader = BufReader::new(map);
    for l in map_reader
        .lines()
        .map(|x| x.expect("Error reading line in map file!"))
    {
        let key = l[0..3].to_string();
        dict.insert(
            l[0..3].to_string(),
            Children {
                left: l[7..10].to_string(),
                right: l[12..15].to_string(),
            },
        );
        if l.chars().nth(2).unwrap() == 'A' {
            current_nodes.push(key.clone());
        }
    }

    println!("{:?}", current_nodes);

    let total = current_nodes
        .into_iter()
        .map(|x| steps_to_z(x, &dict, dir.clone()).unsigned_abs() as u64)
        .reduce(|x, y| lcm(x, y))
        .unwrap();

    return total;
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

use std::cmp::min;
use std::mem::swap;

pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both 2ⁱ u and 2ʲ v
    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} should be odd", u);
        debug_assert!(v % 2 == 1, "v = {} should be odd", v);

        // Swap if necessary so u ≤ v
        if u > v {
            swap(&mut u, &mut v);
        }

        // Identity 4: gcd(u, v) = gcd(u, v-u) as u ≤ v and u, v are both odd
        v -= u;
        // v is now even

        if v == 0 {
            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) as u is odd
        v >>= v.trailing_zeros();
    }
}
