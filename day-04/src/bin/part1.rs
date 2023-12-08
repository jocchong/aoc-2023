use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Scratchcard {
    scratched_numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
}
impl Scratchcard {
    pub fn from(string: String) -> Self {
        let card: String = string
            .chars()
            .skip_while(|c| *c != ':')
            .skip(1)
            .collect::<String>();
        let mut numbers = card.split('|');
        let scratched_numbers: Vec<i32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let winning_numbers: Vec<i32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        Self {
            scratched_numbers,
            winning_numbers,
        }
    }

    fn parse_numbers(str: &str) -> Vec<i32> {
        str.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
    }

    pub fn calculate_score(&self) -> i32 {
        let mut score = 0;
        for number in &self.scratched_numbers {
            if self.winning_numbers.contains(&number) {
                match score {
                    0 => score += 1,
                    _ => score *= 2,
                }
            }
        }
        score
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut sum = 0;
    for line in lines.flatten() {
        let scratchcard = Scratchcard::from(line);
        sum += scratchcard.calculate_score();
    }
    println!("Sum of scratchcard scores is: {}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        let scratchcard = Scratchcard {
            scratched_numbers: vec![41, 48, 83, 86, 17],
            winning_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(scratchcard.calculate_score(), 8);
    }

    #[test]
    fn test_input_parse() {}
}
