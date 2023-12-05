use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut result = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut it = line.split(':').skip(1);
            result += power_min_set(it.next().unwrap());
        }
    }
    dbg!(result);
}

fn power_min_set(game: &str) -> i32 {
    let mut min_set: Vec<i32> = vec![];
    let mut reds: Vec<i32> = vec![];
    let mut blues: Vec<i32> = vec![];
    let mut greens: Vec<i32> = vec![];
    for reveal in game.split(';') {
        for cube in reveal.split(',') {
            let mut cube_it = cube.trim().split(' ');
            let count = cube_it.next().unwrap().parse::<i32>().unwrap();
            match cube_it.next().unwrap() {
                "blue" => blues.push(count),
                "red" => reds.push(count),
                "green" => greens.push(count),
                _ => panic!("bad color"),
            };
        }
    }
    min_set.push(*reds.iter().max().unwrap());
    min_set.push(*blues.iter().max().unwrap());
    min_set.push(*greens.iter().max().unwrap());
    min_set.iter().fold(1, |acc, num| acc * num)
}
