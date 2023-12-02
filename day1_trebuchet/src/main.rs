use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use time::Instant;

#[derive(Debug)]
struct Calibration {
    first: i32,
    last: i32,
    value: i32,
}

impl Calibration {
    fn from(input: &str) -> Self {
        let mut numbers = extract_numbers(input);

        let first = numbers
            .chars()
            .nth(0)
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();

        let last = numbers
            .pop()
            .expect("`numbers` string is empty")
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();

        let value = first * 10 + last;

        Self { first, last, value }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file...");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line..."))
        .collect()
}

fn extract_numbers(input: &str) -> String {
    let mut numbers = String::from("");

    input.chars().for_each(|c| {
        if c.is_ascii_digit() {
            numbers.push(c)
        }
    });

    numbers
}

fn main() {
    let t1 = Instant::now();

    let input = lines_from_file("./data/day1.txt");

    let mut soln = 0;

    let calibs_vec: Vec<_> = input.iter().map(|line| Calibration::from(&line)).collect();

    calibs_vec.iter().for_each(|calib| soln += calib.value);

    println!("{:?}", Instant::now() - t1);

    println!("{:?}", soln);
}
