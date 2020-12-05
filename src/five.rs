use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

pub fn day_five() {
    let file = File::open("input/day5.txt").unwrap();
    let seats = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| seat_number(&x))
        .collect::<Vec<_>>();

    let mut missing = Vec::<(u32, u32)>::new();
    for r in 0..128 {
        for c in 0..8 {
            if !seats.contains(&(r, c)) {
                missing.push((r, c));
            }
        }
    }
    println!("missing: {:?}", missing);
    
}

fn seat_number(s: &str) -> (u32, u32) {
    let mut row = 0..128;
    let mut column = 0..8;
    for c in s.chars() {
        match c {
            'F' => {
                row = lower_half(row);
            },
            'B' => {
                row = upper_half(row);
            },
            'L' => {
                column = lower_half(column);
            },
            'R' => {
                column = upper_half(column);
            },
            _ => panic!("unexpected input {}", c)
        }
    }
    assert_eq!(row.start, row.end-1);
    assert_eq!(column.start, column.end-1);
    (row.start, column.start)
}

fn upper_half(r: Range<u32>) -> Range<u32> {
    (r.start + (r.end - r.start)/2)..r.end
}

fn lower_half(r: Range<u32>) -> Range<u32> {
    r.start..(r.end - (r.end - r.start)/2)
}
