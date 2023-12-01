use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut result = 0;
    for line in lines {
        result += parse_calibration_value(&line.unwrap());
    }
    dbg!(result);
}

fn parse_calibration_value(string: &str) -> i32 {
    let digits_iter: Vec<char> = string.chars().filter(|c| c.is_ascii_digit()).collect();
    let calibration_value = match (digits_iter.first(), digits_iter.last()) {
        (Some(first), Some(second)) => format!("{}{}", first, second),
        _ => panic!("failed to parse line: {}", string),
    };
    calibration_value.parse().unwrap()
}
