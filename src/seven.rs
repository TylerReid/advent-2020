use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day_seven() {
    let file = File::open("input/day7.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut hash = HashMap::<String, Vec<(i32, String)>>::new();
    for line in lines.iter() {
        let parts = line.split(" bags contain ").collect::<Vec<&str>>();
        let key = parts[0];
        let mut values = Vec::<(i32, String)>::new();
        if parts[1] != "no other bags." {
            let value_list = parts[1].split(",").collect::<Vec<&str>>();
            for v in value_list.iter() {
                let x = v.trim().split(" ").collect::<Vec<&str>>();
                values.push((x[0].parse::<i32>().unwrap(), format!("{} {}", x[1], x[2])));
            }
        }
        assert_eq!(false, hash.contains_key(&key.to_string()));
        hash.insert(key.to_string(), values);
    }

    let mut results = HashSet::<String>::new();
    part_one(&hash, &mut results, "shiny gold");
    println!("part one {}", results.len());
    println!("part two {}", part_two(&hash, "shiny gold"));
}

fn part_one(map: &HashMap<String, Vec<(i32, String)>>, collector: &mut HashSet<String>, color: &str) {
    for (k, v) in map.iter() {
        let item = v.iter().find(|&x| x.1 == color);
        match item {
            Some(_) => {
                collector.insert(k.clone());
                part_one(map, collector, k);
            },
            None => (),
        }
    }
}

fn part_two(map: &HashMap<String, Vec<(i32, String)>>, color: &str) -> i32 {
    let mut total = 0;
    match map.get(color) {
        Some(x) => {
            for c in x.iter() {
                total += c.0;
                total += c.0 * part_two(map, &c.1);
            }
        },
        None => (),
    }
    total
}
