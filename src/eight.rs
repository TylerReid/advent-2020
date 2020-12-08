use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

pub fn day_eight() {
    let file = File::open("input/day8.txt").unwrap();
    let ops = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| parse(&x))
        .collect::<Vec<_>>();

    let mut machine = Machine {
        acc: 0,
        pc: 0,
        prog: ops,
    };

    machine.run();
}

#[derive(Debug)]
struct Machine {
    acc: i32,
    pc: usize,
    prog: Vec<Op>,
}

#[derive(Debug, Clone)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop,
}

fn parse(s: &str) -> Op {
    let parts = s.split(" ").collect::<Vec<&str>>();
    match parts[0] {
        "acc" => Op::Acc(parts[1].parse().unwrap()),
        "jmp" => Op::Jmp(parts[1].parse().unwrap()),
        "nop" => Op::Nop,
        x => panic!("bad instruction! {}", x),
    }
}

impl Machine {
    fn run(&mut self) {
        let mut seen = HashSet::<usize>::new();
        loop {
            let i = self.current();
            if seen.contains(&self.pc) {
                println!("have already seen pc: {} op: {:?} acc: {}", self.pc, i, self.acc);
                break;
            }
            seen.insert(self.pc);
            let mut done = false;
            match i {
                Op::Acc(x) => {
                    self.acc += x;
                    done = self.next();
                },
                Op::Jmp(x) => self.pc = add(self.pc, x),
                Op::Nop => done = self.next(),
            }
            if done {
                break;
            }
        }
    }

    fn current(&self) -> Op {
        self.prog[self.pc].clone()
    }

    fn next(&mut self) -> bool {
        self.pc += 1;
        self.pc > self.prog.len()
    }
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize).unwrap()
    } else {
        u.checked_add(i as usize).unwrap()
    }
}
