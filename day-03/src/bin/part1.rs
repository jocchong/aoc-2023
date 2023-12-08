struct EngineSchematic {
    rows: Vec<Vec<char>>,
}

impl EngineSchematic {
    fn new(input: String) -> Self {
        let rows = input
            .lines()
            .map(|s| s.to_string().chars().collect::<Vec<char>>())
            .collect();
        Self { rows }
    }

    fn is_symbol(&self, x: usize, y: usize) -> bool {
        if let Some(row) = &self.rows.get(y) {
            match row.get(x) {
                None => false,
                Some(c) => !c.is_ascii_digit() && *c != '.',
            }
        } else {
            false
        }
    }

    fn is_part_number(&self, x: usize, y: usize) -> Option<String> {
        let mut number = String::new();
        let mut adjacent_to_symbol = false;
        let row = self.rows.get(y).unwrap().iter();
        for (idx, char) in row.skip(x).enumerate() {
            if !char.is_ascii_digit() {
                break;
            }
            number.push(*char);
            for offset_x in idx.saturating_add(x).saturating_sub(1)..=idx.saturating_add(x + 1) {
                for offset_y in y.saturating_sub(1)..=y.saturating_add(1) {
                    if self.is_symbol(offset_x, offset_y) {
                        adjacent_to_symbol = true;
                    }
                }
            }
        }

        if adjacent_to_symbol {
            Some(number)
        } else {
            None
        }
    }

    fn sum_part_numbers(&self) -> i32 {
        let mut sum: i32 = 0;
        for y in 0..self.rows.len() {
            let row = self.rows.get(y).unwrap();
            let mut x = 0;
            while x < row.len() {
                if let Some(part_number) = self.is_part_number(x, y) {
                    sum += part_number.parse::<i32>().unwrap();
                    x += part_number.len();
                }
                x += 1;
            }
        }
        sum
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let engine_schematic = EngineSchematic::new(input);
    let sum = engine_schematic.sum_part_numbers();
    dbg!(sum);
}

#[cfg(test)]
mod tests {
    use super::EngineSchematic;

    #[test]
    fn test_example() {
        let input = String::from("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");

        let engine_schematic = EngineSchematic::new(input);
        let sum = engine_schematic.sum_part_numbers();
        assert_eq!(sum, 4361);
    }
}
