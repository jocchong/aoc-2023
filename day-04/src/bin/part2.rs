use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Card {
    your_numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
}

impl Card {
    pub fn from(string: String) -> Self {
        let (_, body) = string.split_once(':').unwrap();
        let (your_str, winning_str) = body.split_once('|').unwrap();
        let your_numbers: Vec<i32> = your_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let winning_numbers: Vec<i32> = winning_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Self {
            your_numbers,
            winning_numbers,
        }
    }

    pub fn count_matching(&self) -> usize {
        let mut score = 0;
        for number in &self.your_numbers {
            if self.winning_numbers.contains(&number) {
                score += 1;
            }
        }
        score
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let cards: Vec<Card> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| Card::from(line))
        .collect();
    let mut won: Vec<i32> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let matching = card.count_matching();
        for j in 0..matching {
            won[i + j + 1] += won[i];
        }
        dbg!(&won);
    }
    println!("sum: {}", won.iter().sum::<i32>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        let scratchcard = Card {
            your_numbers: vec![41, 48, 83, 86, 17],
            winning_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(scratchcard.count_matching(), 4);
    }
}
