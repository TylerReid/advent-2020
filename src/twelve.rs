use std::fs;

pub fn day_twelve() {
    let actions = fs::read_to_string("input/day12.txt")
        .expect("oh no")
        .lines()
        .map(|x| parse(x))
        .collect::<Vec<Action>>();

    let mut ship = Ship { 
        direction: Direction::East,
        position: (0, 0),
        actions: actions 
    };
    ship.make_it_so();
}

#[derive(Debug)]
struct Ship {
    direction: Direction,
    position: (i32, i32),
    actions: Vec<Action>
}

#[derive(Debug,Clone,Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug,Clone,Copy)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug,Clone,Copy)]
enum Action {
    Direction(Direction, i32),
    Rotation(Rotation, i32),
    Forward(i32),    
}

impl Ship {
    fn manhattan_distance(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }

    fn make_it_so(&mut self) {
        for &a in self.actions.iter() {
            let result = self.take_action(a);
            self.position = result.0;
            self.direction = result.1;
        }
        println!("{}", self.manhattan_distance())
    }

    fn take_action(&self, action: Action) -> ((i32, i32), Direction) {
        match action {
            Action::Direction(d, v) => match d {
                Direction::North => ((self.position.0 + v, self.position.1), self.direction),
                Direction::South => ((self.position.0 - v, self.position.1), self.direction),
                Direction::East => ((self.position.0, self.position.1 + v), self.direction),
                Direction::West => ((self.position.0, self.position.1 - v), self.direction),
            },
            Action::Forward(v) => match self.direction {
                Direction::North => ((self.position.0 + v, self.position.1), self.direction),
                Direction::South => ((self.position.0 - v, self.position.1), self.direction),
                Direction::East => ((self.position.0, self.position.1 + v), self.direction),
                Direction::West => ((self.position.0, self.position.1 - v), self.direction),
            },
            Action::Rotation(r, v) => ((self.position.0, self.position.1), rotate(self.direction, r, v)),
        }
    }
}

//this is dumb, I could put it in an array and + or - based on the rotation, but meh
fn rotate(d: Direction, r: Rotation, v: i32) -> Direction {
    match d {
        Direction::North => match r {
            Rotation::Left => match v {
                90 => Direction::West,
                180 => Direction::South,
                270 => Direction::East,
                _ => panic!("unexpected rotation value {}", v),
            },
            Rotation::Right => match v {
                90 => Direction::East,
                180 => Direction::South,
                270 => Direction::West,
                _ => panic!("unexpected rotation value {}", v),
            },
        },
        Direction::South => match r {
            Rotation::Left => match v {
                90 => Direction::East,
                180 => Direction::North,
                270 => Direction::West,
                _ => panic!("unexpected rotation value {}", v),
            },
            Rotation::Right => match v {
                90 => Direction::West,
                180 => Direction::North,
                270 => Direction::East,
                _ => panic!("unexpected rotation value {}", v),
            },
        },
        Direction::East => match r {
            Rotation::Left => match v {
                90 => Direction::North,
                180 => Direction::West,
                270 => Direction::South,
                _ => panic!("unexpected rotation value {}", v),
            },
            Rotation::Right => match v {
                90 => Direction::South,
                180 => Direction::West,
                270 => Direction::North,
                _ => panic!("unexpected rotation value {}", v),
            },
        },
        Direction::West => match r {
            Rotation::Left => match v {
                90 => Direction::South,
                180 => Direction::East,
                270 => Direction::North,
                _ => panic!("unexpected rotation value {}", v),
            },
            Rotation::Right => match v {
                90 => Direction::North,
                180 => Direction::East,
                270 => Direction::South,
                _ => panic!("unexpected rotation value {}", v),
            },
        },
    }
}

fn parse(s: &str) -> Action {
    let direction = s.chars().next().unwrap();
    let value = s.chars()
        .skip(1).take(s.len() - 1)
        .collect::<String>()
        .parse::<i32>().unwrap();
    
    match direction {
        'N' => Action::Direction(Direction::North, value),
        'S' => Action::Direction(Direction::South, value),
        'E' => Action::Direction(Direction::East, value),
        'W' => Action::Direction(Direction::West, value),
        'L' => Action::Rotation(Rotation::Left, value),
        'R' => Action::Rotation(Rotation::Right, value),
        'F' => Action::Forward(value),
        _ => panic!("bad direction {} from {}", direction, s),
    }
}
