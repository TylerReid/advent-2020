use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub fn day_fourteen() {
    let statements = fs::read_to_string("input/day14.txt")
        .expect("oh no")
        .lines()
        .map(|x| parse(x))
        .collect::<Vec<Statement>>();

    let mut memory = HashMap::<u64, u64>::new();
    let mut current_mask = String::from("nope");

    for s in statements.iter() {
        match s {
            Statement::SetMask(x) => current_mask = String::from(x),
            Statement::WriteMem{mut addr, value} => {
                let mut floating_bits = Vec::new();
                for (i, c) in current_mask.chars().rev().enumerate() {
                    match c {
                        'X' => floating_bits.push(i),
                        '0' => (),
                        '1' => addr |= 1 << i,
                        _ => panic!("bad char {} in {}", c, current_mask),
                    }
                }
                let mut floating_addrs = HashSet::new();
                floating_addrs.insert(addr);
                for fb in floating_bits {
                    //this temp var is here because we can't insert into the hashset while iterating
                    //probably a less dumb way but I don't know how
                    let mut temp = HashSet::new();
                    for a in floating_addrs.iter() {
                        temp.insert(a & !(1 << fb));
                        temp.insert(a | 1 << fb);
                    }
                    for t in temp {
                        floating_addrs.insert(t);
                    }
                    floating_addrs.insert(addr & !(1 << fb));
                    floating_addrs.insert(addr | 1 << fb);
                }

                for a in floating_addrs {
                    memory.insert(a, *value);
                }
            },
        }
    }

    let mut sum = 0;
    for (_, v) in memory {
        sum += v;
    }
    println!("memory sum: {}", sum);
}

#[derive(Debug)]
enum Statement {
    SetMask(String),
    WriteMem {
        addr: u64,
        value: u64,
    },
}

fn parse(s: &str) -> Statement {
    if let Some(x) = MASK.captures(s) {
        return Statement::SetMask(String::from(x.get(1).unwrap().as_str()));
    }

    if let Some(x) =  WRITE_MEM.captures(s) {
        return Statement::WriteMem{
            addr: x.get(1).unwrap().as_str().parse().unwrap(),
            value: x.get(2).unwrap().as_str().parse().unwrap(),
        }
    }

    panic!("invalid statement {}", s);
}

lazy_static! {
    static ref MASK: Regex = Regex::new(r"^mask = ([X10]*)$").unwrap();
    static ref WRITE_MEM: Regex = Regex::new(r"^mem\[(\d*)\] = (\d*)$").unwrap();
}
