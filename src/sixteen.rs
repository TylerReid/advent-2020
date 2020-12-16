use std::fs;
use std::ops::RangeInclusive;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day_sixteen() {
    let input = fs::read_to_string("input/day16.txt").expect("oh no");
    
    let mut rules = Vec::new();
    let mut rule_position: usize = 0;
    let mut my_ticket = Vec::<i64>::new();
    let mut nearby_tickets = Vec::<Vec<i64>>::new();
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
    let mut valid_tickets = Vec::new();
    for ticket in nearby_tickets.iter() {
        let mut all_valid = true;
        for value in ticket.iter() {
            let mut valid = false;
            for rule in rules.iter() {
                if rule.valid_ranges.iter().any(|x| x.contains(value)) {
                    valid = true;
                }
            }
            if !valid {
                all_valid = false;
                invalid_values.push(*value);
            }
        }
        if all_valid {
            valid_tickets.push(ticket.clone());
        }
    }

    let mut rule_positions = Vec::<HashSet<String>>::new();
    for _ in 0..valid_tickets[0].len() {
        let mut hash = HashSet::new();
        for r in rules.iter() {
            hash.insert(r.field.clone());
        }
        rule_positions.push(hash);
    }

    for ticket in valid_tickets.iter() {
        for (i, t) in ticket.iter().enumerate() {
            for r in rules.iter() {
                if !r.valid_ranges.iter().any(|x| x.contains(t)) {
                    rule_positions[i].remove(&r.field);
                }
            }
        }
    }

    let mut inferred = HashSet::new();
    loop {
        let mut temp = rule_positions.clone();
        let mut done = true;
        for i in 0..rule_positions.len() {
            let hash = &rule_positions[i];
            if hash.len() == 1 {
                let f = hash.iter().next().unwrap();
                
                if let None = inferred.get(f) {
                    for j in 0..rule_positions.len() {
                        if i == j {
                            continue;
                        }
                        temp[j].remove(f);
                    }
                    inferred.insert(f.clone());
                }
            } else {
                done = false;
            }
        }
        rule_positions = temp;
        if done {
            break;
        }
    }

    let mut my_values = 1;
    for (i, r) in rule_positions.iter().enumerate() {
        let field = r.iter().next().unwrap();
        if field.starts_with("departure") {
            my_values *= my_ticket[i];
        }
    }
    println!("{}", my_values);
}

#[derive(Debug)]
struct TicketRule {
    field: String,
    position: usize,
    valid_ranges: [RangeInclusive<i64>; 2]
}

lazy_static! {
    static ref RULE: Regex = Regex::new(r"^([\w ]*): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
}
