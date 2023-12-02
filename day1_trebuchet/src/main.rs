use anyhow::Result;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use time::Instant;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file...");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line..."))
        .collect()
}

fn get_first_number(
    input: &String,
    num_words_hmap: &HashMap<&str, i32>,
    digits: &Vec<&str>,
) -> Result<i32> {
    let mut first_idx = 999;
    let mut number = 999;

    for (word, num) in num_words_hmap {
        match input.find(word) {
            Some(idx) => {
                if idx < first_idx {
                    first_idx = idx;
                    number = *num;
                }
            }
            None => continue,
        }
    }

    for num in digits {
        match input.find(num) {
            Some(idx) => {
                if idx < first_idx {
                    first_idx = idx;
                    number = num.parse::<i32>()?;
                }
            }
            None => continue,
        }
    }

    Ok(number)
}

fn get_last_number(
    input: &String,
    num_words_hmap: &HashMap<&str, i32>,
    digits: &Vec<&str>,
) -> Result<i32> {
    let mut last_idx = 0;
    let mut number = 999;

    for (word, num) in num_words_hmap {
        match input.rfind(word) {
            Some(idx) => {
                if idx >= last_idx {
                    last_idx = idx;
                    number = *num;
                }
            }
            None => continue,
        }
    }

    for num in digits {
        match input.rfind(num) {
            Some(idx) => {
                if idx >= last_idx {
                    last_idx = idx;
                    number = num.parse::<i32>()?;
                }
            }
            None => continue,
        }
    }

    Ok(number)
}

fn get_calibration_value(
    input: &String,
    num_words_hmap: &HashMap<&str, i32>,
    digits: &Vec<&str>,
) -> Result<i32> {
    let first = get_first_number(input, num_words_hmap, digits)?;
    let last = get_last_number(input, num_words_hmap, digits)?;

    Ok(first * 10 + last)
}

fn main() {
    let t1 = Instant::now();

    let input = lines_from_file("./data/day1.txt");

    let num_words_hmap = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut soln = 0;

    let calibs_vec: Vec<_> = input
        .iter()
        .map(|line| get_calibration_value(line, &num_words_hmap, &digits).unwrap_or(0))
        .collect();

    calibs_vec.iter().for_each(|value| soln += value);

    println!("{:?}", Instant::now() - t1);

    println!("{:?}", soln);
}
