use std::fs;

pub fn day_one() {
    //todo figure out cross platform path stuff
    let expenses = fs::read_to_string("input/day1.txt")
        .expect("oh no")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    'outer: for i in 0..expenses.len() {
        'inner: for j in (0..expenses.len()).rev() {
            if i == j {
                continue 'inner;
            }

            //part one
            // let sum = expenses[i] + expenses[j];
            // if sum == 2020 {
            //     println!("found it! {} + {} = {}", expenses[i], expenses[j], expenses[i] * expenses[j]);
            //     break 'outer;
            // }
            //part two
            'evenmoreinner: for k in 0..expenses.len() {
                if i == k || j == k {
                    continue 'evenmoreinner;
                }

                let sum = expenses[i] + expenses[j] + expenses[k];
                if sum == 2020 {
                    println!(
                        "found it! {} + {} + {} = {}",
                        expenses[i],
                        expenses[j],
                        expenses[k],
                        expenses[i] * expenses[j] * expenses[k]
                    );
                    break 'outer;
                }
            }
        }
    }
}
