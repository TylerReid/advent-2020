use std::fs;
use std::collections::HashMap;

pub fn day_ten() {
    let mut data = fs::read_to_string("input/day10.txt")
        .expect("oh no")
        .lines()
        .map(|x| (x.parse().unwrap(), Vec::new()))
        .collect::<Vec<(i32, Vec<usize>)>>();

    data.sort();
    let max_laptop_jotlage = data.iter().last().unwrap().0 + 3;

    println!("max laptop jolts: {}", max_laptop_jotlage);
    let mut one_jolt = 0;
    let mut three_jolt = 0;
    let mut last_joltage = 0;
    for jolts in data.iter() {
        let diff = jolts.0 - last_joltage;
        if diff == 1 {
            one_jolt += 1;
        }
        if diff == 3 {
            three_jolt += 1;
        }
        last_joltage = jolts.0;
    }
    //add one more to the three diff since the laptop is alway 3 more
    three_jolt += 1;

    println!("{} * {} = {}", one_jolt, three_jolt, one_jolt * three_jolt);

    for i in (0..data.len()).rev() {
        let current = data[i].0;
        for j in (0..i).rev() {
            let candidate = data[j].0;
            if current - candidate < 4 {
                data[i].1.push(j);
            }
        }
    }

    println!("{:?}", data);

    println!("{:?}", number_of_paths(&data, &mut HashMap::new(), data.len() - 1));

    // let mut combinations: u64 = 1;
    // for i in (0..data.len()).rev() {
    //     let mut possible = 0;
    //     if i == 0 {
    //         possible = 1;
    //     } else {
    //         'inner: for j in (0..i).rev() {
    //             let current = data[i];
    //             let candidate = data[j];
    //             if data[i] - data[j] < 4 {
    //                 println!("{} -> {}", candidate, current);
    //                 possible += 1;
    //             } else {
    //                 break 'inner;
    //             }
    //         }
    //     }
        
    //     if possible == 0 {
    //         println!("{}", i);
    //     }
    //     combinations = combinations * possible;
    // }
    // println!("{}", combinations);
}

fn number_of_paths(data: &Vec<(i32, Vec<usize>)>, cached_paths: &mut HashMap<usize, u64>, i: usize) -> u64 {
    if i == 0 {
        return 1;
    }
    let mut paths = 0;
    for &x in data[i].1.iter() {
        if cached_paths.contains_key(&x) {
            paths += cached_paths.get(&x).unwrap();
        } else {
            let n = number_of_paths(data, cached_paths, x);
            cached_paths.insert(x, n);
            paths += n;
        }
    }
    paths
}