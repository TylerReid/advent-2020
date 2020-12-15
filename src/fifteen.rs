use std::fs;
use std::collections::HashMap;

pub fn day_fifteen() {
    let numbers = fs::read_to_string("input/day15.txt")
        .expect("oh no")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut ages = HashMap::new();
    let mut current_age = 1;
    let mut last_number = 0;
    for &n in numbers.iter() {
        ages.insert(n, current_age);
        last_number = n;
        current_age += 1;
    }

    while current_age <= 30_000_000 {
        if let Some(a) = ages.get(&last_number) {
            let age_diff = (current_age - 1) - a;
            ages.insert(last_number, current_age - 1);
            last_number = age_diff;
        } else {
            ages.insert(last_number, current_age - 1);
            last_number = 0;
        }
        current_age += 1;
    }

    println!("{}", last_number);
}