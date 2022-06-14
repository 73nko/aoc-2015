use std::fs::File;
use std::io::{prelude::*, BufReader};

fn part_one() -> i32 {
    BufReader::new(File::open("./input/day_02.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let sizes = line
                .split('x')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (sizes[0], sizes[1], sizes[2])
        })
        .fold(0, |acc, (l, w, h)| {
            let sorest_side = (l * w).min(w * h).min(l * h);
            acc + 2 * l * w + 2 * w * h + 2 * l * h + sorest_side
        })
}

fn part_two() -> i32 {
    BufReader::new(File::open("./input/day_02.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let sizes = line
                .split('x')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (sizes[0], sizes[1], sizes[2])
        })
        .fold(0, |acc, (l, w, h)| {
            let mut sides: [i32; 3] = [l, w, h];
            sides.sort_unstable();

            let perimeter = sides.iter().take(2).fold(0, |acc, &x| acc + 2 * x);
            acc + perimeter + l * w * h
        })
}

pub fn show() {
    println!("- Part 1: {}", part_one());
    println!("- Part 2: {}", part_two());
}
