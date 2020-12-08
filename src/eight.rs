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

    for (i, op) in ops.iter().enumerate() {
        let mut op_copy = ops.clone();
        match op {
            Op::Acc(_) => (),
            Op::Jmp(x) => op_copy[i] = Op::Nop(*x),
            Op::Nop(x) => op_copy[i] = Op::Jmp(*x),
        }
        
        let mut machine = Machine {
            acc: 0,
            pc: 0,
            prog: op_copy,
        };
    
        if let Some(x) = machine.run() {
            println!("finished with acc: {}", x);
            break;
        }
    }
    
    
}

#[derive(Debug)]
struct Machine {
    acc: i32,
    pc: usize,
    prog: Vec<Op>,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn parse(s: &str) -> Op {
    let parts = s.split(" ").collect::<Vec<&str>>();
    match parts[0] {
        "acc" => Op::Acc(parts[1].parse().unwrap()),
        "jmp" => Op::Jmp(parts[1].parse().unwrap()),
        "nop" => Op::Nop(parts[1].parse().unwrap()),
        x => panic!("bad instruction! {}", x),
    }
}

impl Machine {
    fn run(&mut self) -> Option<i32> {
        let mut seen = HashSet::<usize>::new();
        loop {
            if self.done() {
                return Some(self.acc);
            }
            let i = self.current();
            if seen.contains(&self.pc) {
                return None
            }
            seen.insert(self.pc);
            match i {
                Op::Acc(x) => {
                    self.acc += x;
                    self.next();
                },
                Op::Jmp(x) => self.pc = add(self.pc, x),
                Op::Nop(_) => self.next(),
            }
        }
    }

    fn current(&self) -> Op {
        self.prog[self.pc]
    }

    fn next(&mut self) {
        self.pc += 1;
    }

    fn done(&self) -> bool {
        self.pc >= self.prog.len()
    }
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize).unwrap()
    } else {
        u.checked_add(i as usize).unwrap()
    }
}
