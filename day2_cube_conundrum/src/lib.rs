use regex::Regex;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file...");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line..."))
        .collect()
}

#[derive(Debug)]
pub struct CubeSet {
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>,
}

impl CubeSet {
    fn new(round: &str) -> Self {
        let red_re = Regex::new(r"(?<number>\d+) red").unwrap();
        let blue_re = Regex::new(r"(?<number>\d+) blue").unwrap();
        let green_re = Regex::new(r"(?<number>\d+) green").unwrap();

        let red = match red_re.captures(round) {
            Some(caps) => Some(caps["number"].parse::<i32>().unwrap()),
            None => None,
        };

        let green = match green_re.captures(round) {
            Some(caps) => Some(caps["number"].parse::<i32>().unwrap()),
            None => None,
        };

        let blue = match blue_re.captures(round) {
            Some(caps) => Some(caps["number"].parse::<i32>().unwrap()),
            None => None,
        };

        Self { red, green, blue }
    }
}

#[derive(Debug)]
pub struct Game {
    cubesets: Vec<CubeSet>,
}

impl Game {
    pub fn from(input: &str) -> Self {
        let rounds: Vec<_> = input.split(";").collect();

        let cubesets = rounds.iter().map(|round| CubeSet::new(round)).collect();

        Self { cubesets }
    }

    pub fn is_possible(&self) -> bool {
        for cubeset in &self.cubesets {
            match cubeset.red {
                Some(num) => {
                    if num > 12 {
                        return false;
                    }
                }
                None => (),
            }

            match cubeset.green {
                Some(num) => {
                    if num > 13 {
                        return false;
                    }
                }
                None => (),
            }
            match cubeset.blue {
                Some(num) => {
                    if num > 14 {
                        return false;
                    }
                }
                None => (),
            }
        }
        true
    }

    pub fn max_power(&self) -> i32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for cubeset in &self.cubesets {
            match cubeset.red {
                Some(num) => {
                    if num > max_red {
                        max_red = num;
                    }
                }
                None => (),
            }

            match cubeset.green {
                Some(num) => {
                    if num > max_green {
                        max_green = num;
                    }
                }
                None => (),
            }
            match cubeset.blue {
                Some(num) => {
                    if num > max_blue {
                        max_blue = num;
                    }
                }
                None => (),
            }
        }
        max_red * max_green * max_blue
    }
}
