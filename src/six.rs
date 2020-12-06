use std::fs;
use std::collections::HashSet;

pub fn day_six() {
    let file = fs::read_to_string("input/day6.txt").unwrap();
    let groups: Vec<Vec<HashSet<char>>> = file.split("\n\n")
        .map(|x| create_sets(x))
        .collect();

    let mut total = 0;
    for g in groups {
        let mut intersection: Option<HashSet::<char>> = None;
        for s in g {
            match intersection {
                Some(i) => {
                    
                    intersection = Some(i.union(&s).copied().collect());
                },
                None => {
                    intersection = Some(s);
                },
            }
        }
        total += intersection.unwrap().len();
    }

    println!("total: {}", total)
}

fn create_sets(s: &str) -> Vec<HashSet<char>> {
    let mut sets = Vec::new();
    for l in s.lines() {
        let mut set = HashSet::<char>::new();
        for c in l.chars() {
            set.insert(c);
        }
        sets.push(set);
    }
    sets
}
