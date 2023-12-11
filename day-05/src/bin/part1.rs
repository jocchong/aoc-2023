use std::{cmp, fs, ops::Range};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let almanac = Almanac::parse_input(&input);
    dbg!(almanac.find_min_dest());
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<(Range<usize>, Range<usize>)>>,
}

impl Almanac {
    pub fn find_min_dest(&self) -> usize {
        let seed_locations = self.seeds.iter().map(|seed| self.find_dest(*seed));
        seed_locations.min().unwrap()
    }

    fn find_dest(&self, seed: usize) -> usize {
        let mut dest = seed;
        for map in self.maps.iter() {
            for (dest_r, source_r) in map.iter() {
                if source_r.contains(&dest) {
                    dest = dest_r.start + dest - source_r.start;
                }
            }
        }
        dest
    }

    pub fn parse_input(input: &str) -> Almanac {
        let mut blocks = input.split("\n\n");
        let seeds: Vec<usize> = blocks
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        let mut maps: Vec<Vec<(Range<usize>, Range<usize>)>> = vec![];
        for block in blocks {
            let map: Vec<(Range<usize>, Range<usize>)> = block
                .lines()
                .skip(1)
                .map(|s| Self::parse_ranges(s))
                .collect();
            maps.push(map);
        }
        Almanac { seeds, maps }
    }

    fn parse_ranges(string: &str) -> (Range<usize>, Range<usize>) {
        let mut numbers = string
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap());
        let destination_start = numbers.next().unwrap();
        let source_start = numbers.next().unwrap();
        let range = numbers.next().unwrap();
        (
            destination_start..destination_start + range,
            source_start..source_start + range,
        )
    }
}

