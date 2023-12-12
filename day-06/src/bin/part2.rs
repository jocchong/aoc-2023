use std::time::Duration;

fn main() {
    let input = include_str!("input.txt");
    let record = parse_record(input);
    let answer = count_ways(&record);
    dbg!(answer);
}

#[derive(Debug)]
struct Record(Duration, u128);

fn parse_record(input: &str) -> Record {
    let mut lines = input.lines();
    let time: u64 = lines.next().unwrap().split_whitespace().skip(1).collect::<String>().parse().unwrap();
    let distance: u128 = lines.next().unwrap().split_whitespace().skip(1).collect::<String>().parse().unwrap();
    Record(Duration::from_millis(time), distance)
}

fn count_ways(record: &Record) -> i32 {
    let mut ways = 0;
    for millis_held in 0..record.0.as_millis() {
        let total_distance = millis_held * (record.0.as_millis() - millis_held);
        if total_distance >= record.1 {
            ways += 1;
        }
    }
    ways
}
