use std::fs;

pub fn day_thirteen() {
    let input = fs::read_to_string("input/day13.txt")
        .expect("oh no")
        .lines()
        //need this because the whole String from the file is freed after collect
        //seems like I should be able to do something else
        .map(|x| String::from(x))
        .collect::<Vec<String>>();
    
    let earliest_departure = input[0].parse::<i64>().unwrap();
    let bus_ids = input[1]
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().ok())
        .collect::<Vec<Option<i64>>>();
    
    let mut lowest_time = i64::MAX;
    let mut best_bus = 0;
    for b in bus_ids.iter() {
        if let Some(x) = b {
            let time = earliest_departure - ((earliest_departure % x) - x);
            if time < lowest_time {
                lowest_time = time;
                best_bus = *x;
            }
        }
    }
    println!("{} * {} = {}", best_bus, lowest_time - earliest_departure, best_bus * (lowest_time - earliest_departure));

    let mut bus_index = Vec::new();
    for (i, b) in bus_ids.iter().enumerate() {
        if let &Some(x) = b {
            bus_index.push((x, i as i64));
        }
    }

    bus_index.sort_by(|a, b| a.0.cmp(&b.0));
    bus_index.reverse();

    let mut t = 0;
    loop {
        if departures_line_up(&bus_index, t) {
            println!("{}", t);
            break;
        }
        let mut step = 1;
        // for each consecutive match we find, 
        // we know that the next step has to be a common multiple of all matches
        for b in bus_index.iter() {
            if (t + b.1) % b.0 == 0 {
                step *= b.0;
            }
        }
        t += step;
    }
}

fn departures_line_up(buses: &Vec<(i64, i64)>, t: i64) -> bool {
    for b in buses.iter() {
        if (t + b.1) % b.0  != 0 {
            return false
        }
    }
    true
}
