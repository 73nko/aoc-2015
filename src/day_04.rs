use md5;
use std::fs::File;
use std::io::{BufReader, Read};

fn read_input() -> String {
    let mut input = String::new();
    BufReader::new(File::open("./input/day_04.txt").unwrap())
        .read_to_string(&mut input)
        .unwrap();
    input
}

fn solution(prefix: &str) -> i32 {
    let input = read_input();
    let mut result = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, result).as_bytes());
        if (format!("{:x}", hash)).starts_with(prefix) {
            return result;
        }
        result += 1;
    }
}
pub fn show() {
    println!("- Part 1: {}", solution("00000"));
    println!("- Part 2: {}", solution("000000"));
}
