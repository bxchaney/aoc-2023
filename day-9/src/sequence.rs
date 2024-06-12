use std::{fs::File, io::prelude::*, io::BufReader};

pub fn sequence(file: File) -> (i32, i32) {
    let reader = BufReader::new(file);
    let mut pt1_total = 0;
    let mut pt2_total = 0;
    for l in reader
        .lines()
        .map(|x| x.expect("problem opening reading file!"))
    {
        pt1_total += get_next(parse(&l));
        pt2_total += get_prev(parse(&l));
    }
    return (pt1_total, pt2_total);
}

fn parse(line: &str) -> Vec<i32> {
    line.split(' ')
        .map(|x| x.parse::<i32>().expect("error parsing value!"))
        .collect()
}

fn get_next(seq: Vec<i32>) -> i32 {
    let mut difference_sequence = vec![];
    let mut zero_count = 0;
    for i in 1..seq.len() {
        let difference = seq[i] - seq[i - 1];
        if difference == 0 {
            zero_count += 1;
        }
        difference_sequence.push(difference);
    }
    if zero_count == difference_sequence.len() {
        return 0 + seq[seq.len() - 1];
    } else {
        return seq[seq.len() - 1] + get_next(difference_sequence);
    }
}

fn get_prev(seq: Vec<i32>) -> i32 {
    let mut difference_sequence = vec![];
    let mut zero_count = 0;
    for i in 1..seq.len() {
        let difference = seq[i] - seq[i - 1];
        if difference == 0 {
            zero_count += 1;
        }
        difference_sequence.push(difference);
    }
    if zero_count == difference_sequence.len() {
        return seq[0];
    } else {
        return seq[0] - get_prev(difference_sequence);
    }
}
