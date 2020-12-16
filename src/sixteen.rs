use std::fs;
use std::ops::RangeInclusive;
use regex::Regex;

pub fn day_sixteen() {
    let input = fs::read_to_string("input/day16.txt").expect("oh no");
    
    let mut rules = Vec::new();
    let mut rule_position: usize = 0;
    let mut my_ticket = Vec::<i32>::new();
    let mut nearby_tickets = Vec::<Vec<i32>>::new();
    let mut begin_my_ticket = false;
    let mut begin_nearby = false;

    for l in input.lines() {
        if let Some(x) = RULE.captures(l) {
            rules.push(TicketRule {
                field: x.get(1).unwrap().as_str().to_string(),
                position: rule_position,
                valid_ranges: [
                    x[2].parse().unwrap()..=x[3].parse().unwrap(),
                    x[4].parse().unwrap()..=x[5].parse().unwrap(),
                ]
            });
            rule_position += 1;
        }

        if begin_my_ticket {
            for i in l.split(",") {
                my_ticket.push(i.parse().unwrap());
            }
        }
        begin_my_ticket = l == "your ticket:";

        if begin_nearby {
            let mut t = Vec::new();
            for i in l.split(",") {
                t.push(i.parse().unwrap());
            }
            nearby_tickets.push(t);
        }
        begin_nearby = if l == "nearby tickets:" { true } else { begin_nearby };
    }

    let mut invalid_values = Vec::new();
    for ticket in nearby_tickets.iter() {
        for (i, value) in ticket.iter().enumerate() {
            let mut valid = false;
            for rule in rules.iter() {
                if rule.valid_ranges.iter().any(|x| x.contains(value)) {
                    valid = true;
                }
            }
            if !valid {
                invalid_values.push(*value);
            }
        }
    }

    println!("{:?} {}", invalid_values, invalid_values.iter().sum::<i32>());
}

#[derive(Debug)]
struct TicketRule {
    field: String,
    position: usize,
    valid_ranges: [RangeInclusive<i32>; 2]
}

lazy_static! {
    static ref RULE: Regex = Regex::new(r"^(\w*): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
}
