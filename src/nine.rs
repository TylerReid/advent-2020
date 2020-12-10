use std::fs;

pub fn day_nine() {
    let data = fs::read_to_string("input/day9.txt")
        .expect("oh no")
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut start: usize = 0;
    let mut end: usize = 25;
    let found_index: usize;
    let found_value: u64;

    loop {
        let preamble = &data[start..end];
        if !is_sum_of_two(preamble, data[end]) {
            found_index = end;
            found_value = data[end];
            println!("{} is not the sum of {:?}", data[end], preamble);
            break;
        }
        start += 1;
        end += 1;
    }

    'outer: for x in 0..found_index {
        let mut sum = 0;
        for y in x..found_index {
            sum += data[y];
            if sum == found_value {
                let r = &data[x..y + 1];
                println!("found values: {:?}", &data[x..y + 1]);
                println!(
                    "weakness: {}",
                    r.iter().min().unwrap() + r.iter().max().unwrap()
                );
                break 'outer;
            }
        }
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
