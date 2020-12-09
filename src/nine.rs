use std::fs;
use std::ops::Range;

pub fn day_nine() {
    let data = fs::read_to_string("input/day9.txt")
        .expect("oh no")
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut start: usize = 0;
    let mut end: usize = 25;

    loop {
        let preamble = &data[start..end];
        if !is_sum_of_two(preamble, data[end]) {
            println!("{} is not the sum of {:?}", data[end], preamble);
            break;
        }
        start += 1;
        end += 1;
    }
}

fn is_sum_of_two(r: &[u64], v: u64) -> bool {
    for n in r.iter() {
        for x in r.iter() {
            if n == x {
                continue;
            }
            if n + x == v {
                return true;
            }
        }
    }

    false
}


