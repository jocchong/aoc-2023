fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let times: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .flatten()
        .collect();
    let distances: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .flatten()
        .collect();
    dbg!(times, distances);
}
