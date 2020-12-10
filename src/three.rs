use std::fs::File;
use std::io::{self, BufRead};

pub fn day_three() {
    let file = File::open("input/day3.txt").unwrap();
    let rows = io::BufReader::new(file)
        .lines()
        .map(|x| parse(&x.unwrap()))
        .collect::<Vec<_>>();

    let one = tree_hit_count(&rows, 1, 1);
    let two = tree_hit_count(&rows, 3, 1);
    let three = tree_hit_count(&rows, 5, 1);
    let four = tree_hit_count(&rows, 7, 1);
    let five = tree_hit_count(&rows, 1, 2);

    println!("{} trees hit", one * two * three * four * five);
}

fn tree_hit_count(rows: &Vec<Vec<Square>>, right: usize, down: usize) -> u32 {
    let mut tree_count = 0;
    let mut h = 0;
    let row_length = rows[0].len();

    let mut i = 0;
    while i < rows.len() {
        match &rows[i][h] {
            Square::Tree => tree_count += 1,
            _ => (),
        }

        h = (h + right) % row_length;
        i += down
    }
    tree_count
}

fn parse(s: &str) -> Vec<Square> {
    let mut v = Vec::new();
    for c in s.chars() {
        match c {
            '.' => v.push(Square::Open),
            '#' => v.push(Square::Tree),
            _ => panic!("unknown char {}", c),
        }
    }
    v
}

enum Square {
    Open,
    Tree,
}
