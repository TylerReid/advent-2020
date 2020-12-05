use std::fs::File;
use std::io::{self, BufRead};

pub fn day_five() {
    let file = File::open("input/day5.txt").unwrap();
    let mut seats = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| parse(&x))
        .collect::<Vec<_>>();
    
    println!("max: {}", seats.iter().max().unwrap());
    seats.sort();
    for i in 0..seats.len() {
        if seats[i] != seats[i + 1] - 1 {
            println!("my seat: {}", seats[i + 1] -1);
            break;
        }
    }
}

fn parse(s: &str) -> i32 {
    let mut n = 0;
    let mut i = 0;
    for c in s.chars().rev() {
        let mask = 1 << i;
        if c == 'B' || c == 'R' {
            n |= mask;
        }
        i += 1;
    }
    n
}