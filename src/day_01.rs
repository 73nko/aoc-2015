use std::fs::File;
use std::io::{BufReader, Read};

fn part_one() -> i32 {
    BufReader::new(File::open("./input/day_01.txt").unwrap())
        .bytes()
        .map(|b| b.unwrap() as char)
        .fold(0, |acc, x| if x == '(' { acc + 1 } else { acc - 1 })
}

fn part_two() -> i32 {
    let mut position: i32 = -1;
    BufReader::new(File::open("./input/day_01.txt").unwrap())
        .bytes()
        .map(|b| b.unwrap() as char)
        .enumerate()
        .fold(0, |acc, (i, x)| {
            let r = if x == '(' { acc + 1 } else { acc - 1 };
            if r == -1 && position < 0 {
                position = i as i32 + 1;
            }
            r
        });

    position
}

pub fn show() {
    println!("- Part 1: {}", part_one());
    println!("- Part 2: {}", part_two());
}
