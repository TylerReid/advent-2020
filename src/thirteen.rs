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

    // bus_index is a list of (bus_id, index it is in the original list)
    // to make this as fast as possible,
    // sort this list so the biggest id is first 
    // because we are going to be jumping forward in the search space using this id in the order of this list 
    // and we want to jump as far as possible
    bus_index.sort_by(|a, b| a.0.cmp(&b.0));
    bus_index.reverse();

    let mut t = 0;
    loop {
        // if this is the case, we are done looking
        if departures_line_up(&bus_index, t) {
            println!("{}", t);
            break;
        }
        // the amount we are going to jump forward each iteration
        let mut step = 1;
        // for each consecutive match we find, 
        // we know that the next step has to be a common multiple of all matches
        // this is because the bus ids have no common divisors
        // so when bus a and b up has to be at each a * b time step
        // each time we find the next bus in the sequence matches, step will increase to step * bus_id
        for b in bus_index.iter() {
            // check if the time + the offset is divisible by 0, meaning it lines up
            if (t + b.1) % b.0 == 0 {
                // multiply the step we have by the matches
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
