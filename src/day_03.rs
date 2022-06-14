use std::fs::File;
use std::io::{BufReader, Read};

fn get_position(position: [i32; 2], direction: char) -> [i32; 2] {
    match direction {
        '^' => [position[0] - 1, position[1]],
        'v' => [position[0] + 1, position[1]],
        '<' => [position[0], position[1] - 1],
        '>' => [position[0], position[1] + 1],
        _ => panic!("Unknown direction: {}", direction),
    }
}

fn part_one() -> i32 {
    let mut current_position = [0, 0];
    let mut positions = std::collections::HashSet::new();

    positions.insert(current_position);
    BufReader::new(File::open("./input/day_03.txt").unwrap())
        .bytes()
        .for_each(|x| {
            current_position = get_position(current_position, x.unwrap() as char);
            positions.insert(current_position);
        });

    positions.len() as i32
}

fn part_two() -> i32 {
    BufReader::new(File::open("./input/day_03.txt").unwrap());

    1
}

pub fn show() {
    println!("- Part 1: {}", part_one());
    //println!("- Part 2: {}", part_two());
}
