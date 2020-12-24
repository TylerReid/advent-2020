use std::fs;

pub fn f() {
    let input = "327465189";

    let mut cups = input.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let mut current_cup: usize = 0;

    for i in 1..101 {
        current_cup = do_move(&mut cups, current_cup, i);
    }

    println!("final: {} - {:?}", current_cup, cups);
    let one_index = cups.iter().position(|&x| x == 1).unwrap();
    for i in 1..9 {
        print!("{}", cups[(one_index + i) % cups.len()]);
    }
}

fn do_move(cups: &mut Vec<u32>, current_cup: usize, move_number: u32) -> usize {
    //set up data needed later before cups is modified
    let mut current_cup_value = cups[current_cup];
    let mut destination = cups[current_cup] - 1;
    let lowest_cup = cups.iter().min().unwrap().clone();
    let highest_cup = cups.iter().max().unwrap().clone();
    //print debug info
    println!("-- move {} --", move_number);
    print!("cups: ");
    for (i, c) in cups.iter().enumerate() {
        if i == current_cup {
            print!("({}) ", c);
        } else {
            print!("{} ", c);
        }
    }
    println!("");
    //pick out next 3 cups
    let mut pickup_index = (current_cup + 1) % cups.len();
    let mut pickup_cups = Vec::new();
    for _ in 0..3 {
        if pickup_index >= cups.len() {
            pickup_index = 0
        }
        pickup_cups.push(cups.remove(pickup_index));
    }
    print!("pick up: ");
    for p in pickup_cups.iter() {
        print!("{}, ", p);
    }
    println!("");
    //get destination
    destination = loop {
        if destination < lowest_cup {
            destination = highest_cup;
            continue;
        }
        if pickup_cups.contains(&destination) {
            destination -= 1;
            continue;
        }
        break destination;
    };
    println!("desination: {}", destination);

    let insert_index = cups.iter()
        .position(|&x| x == destination).unwrap() + 1;
    //insert cups
    for i in 0..3 {
        cups.insert(insert_index + i, pickup_cups[i]);
    }
    println!("");
    (cups.iter().position(|&x| x == current_cup_value).unwrap() + 1) % cups.len()
}