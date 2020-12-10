use std::fs;
use std::collections::HashMap;

pub fn day_ten() {
    let mut data = fs::read_to_string("input/day10.txt")
        .expect("oh no")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    //push a zero because the wall is zero, but not included in the list
    data.push(0);
    data.sort();
    let laptop_jotlage = data.iter().last().unwrap() + 3;

    println!("laptop jolts: {}", laptop_jotlage);
    let mut one_jolt = 0;
    let mut three_jolt = 0;
    let mut last_joltage = 0;
    for &jolts in data.iter() {
        let diff = jolts - last_joltage;
        if diff == 1 {
            one_jolt += 1;
        }
        if diff == 3 {
            three_jolt += 1;
        }
        last_joltage = jolts;
    }
    //add one more to the three diff since the laptop is always 3 more
    three_jolt += 1;
    println!("part one: {} * {} = {}", one_jolt, three_jolt, one_jolt * three_jolt);

    let mut edges =std::iter::repeat(vec![])
        .take(data.len())
        .collect::<Vec<Vec<usize>>>();

    for i in (0..data.len()).rev() {
        let current = data[i];
        for j in (0..i).rev() {
            let candidate = data[j];
            if current - candidate < 4 {
                edges[i].push(j);
            }
        }
    }

    println!("part two: {:?}", number_of_paths(&edges, &mut HashMap::new(), data.len() - 1));
}

fn number_of_paths(data: &Vec<Vec<usize>>, cached_paths: &mut HashMap<usize, u64>, i: usize) -> u64 {
    if i == 0 {
        return 1;
    }
    let mut paths = 0;
    for &x in data[i].iter() {
        if let Some(p) = cached_paths.get(&x) {
            paths += p;
        } else {
            let n = number_of_paths(data, cached_paths, x);
            cached_paths.insert(x, n);
            paths += n;
        }
    }
    paths
}