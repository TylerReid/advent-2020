use std::fs;
use std::collections::HashMap;

const number_of_cups: usize = 1_000_001;
const lowest_cup: usize = 1;
const highest_cup: usize = 1_000_000;

pub fn f() {
    let input = "327465189";
    let mut current_cup = input.chars().next().unwrap().to_digit(10).unwrap() as usize;
    let mut cups = [0; number_of_cups];
    let mut chars = input.chars().peekable();
    while let Some(x) = chars.next() {
        let current = x.to_digit(10).unwrap() as usize;
        let next = match chars.peek() {
            Some(y) => y.to_digit(10).unwrap() as usize,
            None => 10,
        };
        cups[current] = next;
    }

    for i in 10..=1_000_000 {
        cups[i] = if i + 1 == number_of_cups { current_cup } else { i + 1 }
    }    

    for i in 1..=10_000_000 {
        current_cup = do_move(&mut cups, current_cup, i);
    }

    let one = cups[1];
    let two = cups[one];

    println!("{} * {} = {}", one, two, one * two);
}

fn do_move(cups: &mut [usize; number_of_cups], current_cup: usize, move_number: u32) -> usize {
    //pick out next 3 cups
    //println!("-- move {} --", move_number);
    //print!("cups: ");
    //print_cups(cups, current_cup);
    
    let pickup_one = cups[current_cup];
    let pickup_two = cups[pickup_one];
    let pickup_three = cups[pickup_two];
    //println!("pick up: {} {} {}", pickup_one, pickup_two, pickup_three);
    //cut the three out of the chain
    cups[current_cup] = cups[pickup_three];

    //get destination
    let mut destination = current_cup - 1;
    destination = loop {
        if destination < lowest_cup {
            destination = highest_cup;
            continue;
        }
        if destination == pickup_one || destination == pickup_two || destination == pickup_three {
            destination -= 1;
            continue;
        }
        break destination;
    };
    //println!("destination: {}", destination);
    //insert cups
    let end = cups[destination];
    cups[destination] = pickup_one;
    cups[pickup_one] = pickup_two;
    cups[pickup_two] = pickup_three;
    cups[pickup_three] = end;

    cups[current_cup]
}

fn print_cups(cups: &[usize; number_of_cups], start: usize) {
    print!("({}) ", start);
    let mut next = cups[start];
    while next != start {
        print!("{} ", next);
        next = cups[next];
    }
    println!("");
}
