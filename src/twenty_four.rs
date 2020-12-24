use std::fs;
use std::collections::HashSet;

pub fn f() {
    let input = fs::read_to_string("input/day24.txt")
        .expect("oh no");

    let mut instruction_sets = Vec::new();
    for l in input.lines() {
        instruction_sets.push(parse(l));
    }

    let mut black_tiles = HashSet::new();
    for set in instruction_sets.iter() {
        let mut tile = (0, 0, 0);
        for i in set.iter() {
            tile = do_instruction(&tile, i);
        }
        if black_tiles.contains(&tile) {
            println!("already contains {:?}", tile);
            black_tiles.remove(&tile);
        } else {
            println!("new tile {:?}", tile);
            black_tiles.insert(tile);
        }
        println!("count: {}", black_tiles.len());
    }

    for i in 0..100 {
        let mut next_tiles = HashSet::new();
        for t in black_tiles.iter() {
            let w = white_neighbors(&black_tiles, t);
            if w.len() == 5 || w.len() == 4 {
                next_tiles.insert(*t);
            }
            for x in w.iter() {
                if white_neighbors(&black_tiles, x).len() == 4 {
                    next_tiles.insert(*x);
                }
            }
        }
        black_tiles = next_tiles;
    }

    println!("{}", black_tiles.len());
}

type Tile = (i32, i32, i32);

#[derive(Debug)]
enum Instruction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn do_instruction(t: &Tile, i: &Instruction) -> Tile {
    let mut new = t.clone();
    match i {
        Instruction::E => {
            new.0 += 1;
            new.1 -= 1;
        },
        Instruction::SE => {
            new.2 += 1;
            new.1 -= 1;
        },
        Instruction::SW => {
            new.0 -= 1;
            new.2 += 1;
        },
        Instruction::W => {
            new.0 -= 1;
            new.1 += 1;
        },
        Instruction::NW => {
            new.1 += 1;
            new.2 -= 1;
        },
        Instruction::NE => {
            new.0 += 1;
            new.2 -= 1;
        },
    }
    new
}

fn white_neighbors(black_tiles: &HashSet<Tile>, tile: &Tile) -> Vec<Tile> {
    let mut v = Vec::new();

    let neighbors = vec![
        do_instruction(tile, &Instruction::E),
        do_instruction(tile, &Instruction::SE),
        do_instruction(tile, &Instruction::SW),
        do_instruction(tile, &Instruction::W),
        do_instruction(tile, &Instruction::NW),
        do_instruction(tile, &Instruction::NE),
    ];

    for n in neighbors {
        if !black_tiles.contains(&n) {
            v.push(n);
        }
    }

    v
}

fn parse(s: &str) -> Vec<Instruction> {
    let mut v = Vec::new();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        match c {
            'e' => v.push(Instruction::E),
            's' => {
                let next = chars.next().unwrap();
                match next {
                    'w' => v.push(Instruction::SW),
                    'e' => v.push(Instruction::SE),
                    _ => panic!("unexpected char {}", next),
                }
            },
            'n' => {
                let next = chars.next().unwrap();
                match next {
                    'w' => v.push(Instruction::NW),
                    'e' => v.push(Instruction::NE),
                    _ => panic!("unexpected char {}", next),
                }
            },
            'w' => v.push(Instruction::W),
            _ => panic!("unexpected char {}", c),
        }
    }

    v
}
