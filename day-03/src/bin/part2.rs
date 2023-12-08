use std::fs;

fn main() {
    let schematic: String = fs::read_to_string("input.txt").unwrap();
    dbg!(calculate_sum(schematic));
}

fn calculate_sum(input: String) -> i32 {
    let schematic: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let mut sum_of_ratios = 0;

    for (y, row) in schematic.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '*' {
                let mut first_num = String::new();
                let mut second_num = String::new();
                for ny in y.saturating_sub(1)..y.saturating_add(1) {
                    for nx in x.saturating_sub(1)..x.saturating_add(1) {
                        if let Some(row) = schematic.get(ny) {
                        }
                    }
                }
            }
        }
    }
    sum_of_ratios
}

#[cfg(test)]
mod tests {
    use crate::calculate_sum;

    #[test]
    fn test_example() {
        let input = String::from(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        let sum = calculate_sum(input);
        assert_eq!(sum, 467835);
    }
}
