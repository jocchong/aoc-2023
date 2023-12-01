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
    let digits_iter: Vec<char> = convert_spelled_to_digits(string)
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    let calibration_value = match (digits_iter.first(), digits_iter.last()) {
        (Some(first), Some(second)) => format!("{}{}", first, second),
        _ => panic!("failed to parse line: {}", string),
    };
    calibration_value.parse().unwrap()
}

fn convert_spelled_to_digits(string: &str) -> String {
    let mut result = String::new();
    for (i, char) in string.chars().enumerate() {
        let mut number_found = false;
        for offset in 3..6 {
            if let Some(slice) = slice_safe(string, i, i + offset) {
                if let Some(digit) = spelled_to_digit(slice) {
                    result.push(digit);
                    number_found = true;
                    break;
                }
            }
        }
        if !number_found {
            result.push(char);
        }
    }
    result
}

fn slice_safe(string: &str, start: usize, end: usize) -> Option<&str> {
    if start > end || end > string.len() {
        None
    } else {
        Some(&string[start..end])
    }
}

fn spelled_to_digit(string: &str) -> Option<char> {
    match string {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}
