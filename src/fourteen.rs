use std::fs;
use std::collections::HashMap;
use regex::Regex;

pub fn day_fourteen() {
    let statements = fs::read_to_string("input/day14.txt")
        .expect("oh no")
        .lines()
        .map(|x| parse(x))
        .collect::<Vec<Statement>>();

    println!("{:?}", statements);

    let mut memory = HashMap::<u64, u64>::new();
    let mut current_mask = String::from("nope");

    for s in statements.iter() {
        match s {
            Statement::SetMask(x) => current_mask = String::from(x),
            Statement::WriteMem{addr, mut value} => {
                println!("mask: {}\nbits: {:036b}", current_mask, value);
                for (i, c) in current_mask.chars().rev().enumerate() {
                    match c {
                        'X' => (),
                        '0' => value &= !(1 << i),
                        '1' => value |= 1 << i,
                        _ => panic!("bad char {} in {}", c, current_mask),
                    }
                }
                memory.insert(*addr, value);
                println!("res:  {:036b}\n", value);
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
