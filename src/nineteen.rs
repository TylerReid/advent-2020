use std::fs;
use std::collections::HashMap;
use regex::Regex;

pub fn day_nineteen() {
    let input = fs::read_to_string("input/day19.txt").expect("oh no");
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let rules = parse_rules(parts[0]);
    println!("{:?}", rules);

    let mut num_follow = 0;
    for l in parts[1].lines() {
        //num_follow += if follows_rules(&rules, rules.get(&0).unwrap(), l) { 1 } else { 0 };
    }

    println!("{}", num_follow);
}

fn parse_rules(s: &str) -> HashMap<u8, Rule> {
    let mut map = HashMap::new();
    for l in s.lines() {
        let r = parse_rule(l);
        map.insert(r.id(), r);
    }

    map
}

fn parse_rule(s: &str) -> Rule {
    if let Some(x) = TERMINAL_RULE.captures(s) {
        return Rule::Terminal {
            id: x.get(1).unwrap().as_str().parse().unwrap(),
            value: x.get(2).unwrap().as_str().chars().next().unwrap(), //todo seems dumb
        }
    }

    if let Some(x) = CONTAINER_RULE.captures(s) {
        let id = x.get(1).unwrap().as_str().parse().unwrap();
        let rules = x.get(2).unwrap().as_str()
            .split("|")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let rule_set = if rules.len() == 1 {
            RuleSet::Single(
                rules[0].chars().nth(0).unwrap().to_digit(10).unwrap() as u8, 
                rules[0].chars().nth(2).unwrap().to_digit(10).unwrap() as u8
            )
        } else if rules.len() == 2 {
            RuleSet::Double(
            (
                rules[0].chars().nth(0).unwrap().to_digit(10).unwrap() as u8, 
                rules[0].chars().nth(2).unwrap().to_digit(10).unwrap() as u8
            ),
            (
                rules[1].chars().nth(0).unwrap().to_digit(10).unwrap() as u8, 
                rules[1].chars().nth(2).unwrap().to_digit(10).unwrap() as u8
            ))
        } else {
            panic!("unexpected rule {}", s)
        };

        return Rule::Container {
            id: id,
            rule_set: rule_set,
        }
    }

    panic!("rule didn't match patterns {}", s);
}

#[derive(Debug)]
enum Rule {
    Container {
        id: u8,
        rule_set: RuleSet,
    },
    Terminal {
        id: u8,
        value: char,
    },
}

#[derive(Debug)]
enum RuleSet {
    Single(u8, u8),
    Double((u8, u8), (u8, u8)),
}

impl Rule {
    // seems silly, think about if I could structure the types better
    fn id(&self) -> u8 {
        match self {
            Rule::Container{id, ..} => *id,
            Rule::Terminal{id, ..} => *id, 
        }
    }
}

lazy_static! {
    static ref TERMINAL_RULE: Regex = Regex::new(r#"^(\d*): "(\w)"$"#).unwrap();
    static ref CONTAINER_RULE: Regex = Regex::new(r#"^(\d*): (.*)$"#).unwrap();
}
