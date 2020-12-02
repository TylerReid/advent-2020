extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

lazy_static! {
    static ref R: Regex = Regex::new("^(\\d*)-(\\d*) (.): (.*)$").unwrap();
}

#[allow(dead_code)]
pub fn day_two() {
    let file = File::open("input/day2.txt").unwrap();

    let raw_passwords = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    
    let mut valid = 0;

    for pw in raw_passwords.iter() {
        let p = parse_entry(pw);
        if p.is_valid() {
            valid += 1;
        }
    }
    println!("{} valid passwords", valid);
}

fn parse_entry(s: &str) -> PasswordEntry {
    let captures = R.captures(s).unwrap();
    PasswordEntry {
        required_letter: captures[3].chars().next().unwrap(),
        min: captures[1].parse().unwrap(),
        max: captures[2].parse().unwrap(),
        value: captures[4].to_string(),
    }
}

#[derive(Debug)]
struct PasswordEntry {
    required_letter: char,
    min: usize,
    max: usize,
    value: String,
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let n = self.value
            .matches(self.required_letter)
            .count();
        self.min <= n && n <= self.max
    }
}