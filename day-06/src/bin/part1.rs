use std::time::Duration;

fn main() {
    let input = include_str!("input.txt");
    let records = parse_records(input);
    let answer = records.iter().fold(1, |accumulator, record| { accumulator * count_ways(record)});
    dbg!(answer);
}

#[derive(Debug)]
struct Record(Duration, u128);

fn parse_records(input: &str) -> Vec<Record> {
    let mut lines = input.lines();
    let mut records: Vec<Record> = vec![];
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .flatten()
        .collect();
    let distances: Vec<u128> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .flatten()
        .collect();
    assert_eq!(times.len(), distances.len());
    for (index, time) in times.iter().enumerate() {
        let distance = distances.get(index).unwrap();
        records.push(Record(Duration::from_millis(*time), *distance));
    }
    records
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
