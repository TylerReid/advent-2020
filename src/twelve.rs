use std::fs;

pub fn day_twelve() {
    let actions = fs::read_to_string("input/day12.txt")
        .expect("oh no")
        .lines()
        .map(|x| parse(x))
        .collect::<Vec<Action>>();

    let mut ship = Ship {
        position: (0, 0),
        waypoint: (10, 1),
    };
    ship.make_it_so(actions);
}

#[derive(Debug)]
struct Ship {
    position: (i32, i32),
    waypoint: (i32, i32),
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

    fn make_it_so(&mut self, actions: Vec<Action>) {
        for &a in actions.iter() {
            self.take_action(a);
        }
        println!("{}", self.manhattan_distance())
    }

    fn take_action(&mut self, action: Action) {
        match action {
            Action::Direction(d, v) => match d {
                Direction::North => self.waypoint = (self.waypoint.0, self.waypoint.1 + v),
                Direction::South => self.waypoint = (self.waypoint.0, self.waypoint.1 - v),
                Direction::East => self.waypoint = (self.waypoint.0 + v, self.waypoint.1),
                Direction::West => self.waypoint = (self.waypoint.0 - v, self.waypoint.1),
            },
            Action::Forward(v) => self.position = forward(self.position, self.waypoint, v),
            Action::Rotation(r, v) => self.waypoint = rotate(r, self.waypoint, v),
        };
    }
}

fn forward(mut pos: (i32, i32), waypoint: (i32, i32), times: i32) -> (i32, i32) {
    println!("start pos {:?} to waypoint {:?} {} times", pos, waypoint, times);
    for i in 0..times {
        pos = (pos.0 + waypoint.0, pos.1 + waypoint.1);
        println!("next pos {:?}", pos);
    }
    pos
}

fn rotate(r: Rotation, waypoint: (i32, i32), v: i32) -> (i32, i32) {
    match v {
        90 => match r {
            Rotation::Left => (-waypoint.1, waypoint.0),
            Rotation::Right => (waypoint.1, -waypoint.0),
        },
        180 => (-waypoint.0, -waypoint.1),
        270 => match r {
            Rotation::Left => (waypoint.1, -waypoint.0),
            Rotation::Right => (-waypoint.1, waypoint.0),
        },
        _ => panic!("unexpected rotation {}", v),
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
