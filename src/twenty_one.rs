use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn f() {
    let input = fs::read_to_string("input/day21.txt")
        .expect("oh no");
    let lines = input.lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut ingredients = Vec::new();
    for l in lines.iter() {
        ingredients.push(parse(l));
    }

    let mut ingredient_counts = HashMap::<&str, u32>::new();
    for ingredient in ingredients.iter() {
        for i in ingredient.ingredients.iter() {
            if let Some(x) = ingredient_counts.get_mut(i) {
                *x += 1;
            } else {
                ingredient_counts.insert(i, 1);
            }
        }
    }

    let (all_allergens, all_ingredients) = all_allergens_ingredients(&ingredients);
    let mut possibilites = create_possible_allergens(&ingredients, &all_allergens, &all_ingredients);

    loop {
        let mut single_possibilities = HashMap::new();
        for (&a, i) in possibilites.iter() {
            if i.len() == 1 {
                single_possibilities.insert(a.clone(), i.clone());
            }
        }

        for (&a, i) in possibilites.iter_mut() {
            for (&s, ing) in single_possibilities.iter() {
                if a != s {
                    i.remove(ing.iter().next().unwrap());
                }
            }
        }

        if single_possibilities.len() == possibilites.len() {
            break;
        }
    }

    let mut single_possibilities = HashSet::new();
    for (_, i) in possibilites.iter() {
        single_possibilities.insert(*i.iter().next().unwrap());
    }

    let impossibles = all_ingredients.difference(&single_possibilities);

    let mut match_count = 0;
    for &impossible in impossibles {
        match_count += ingredient_counts[impossible];
    }
    println!("{}", match_count);
    println!("{:?}", possibilites);
}

fn parse(s: &str) -> Ingredients {
    if let Some(x) = INGREDIENTS_ALERGENS.captures(s) {
        let allergens = if let Some(a) = x.get(3) {
            a.as_str().split(", ").collect()
        } else {
            HashSet::new()
        };
        Ingredients {
            ingredients: x.get(1).unwrap().as_str().split_terminator(" ").collect(),
            allergens: allergens,
        }
    } else {
        panic!("unexpected input {}", s)
    }
}

fn all_allergens_ingredients<'a>(ingredients: &'a Vec<Ingredients>) -> (HashSet<&'a str>, HashSet<&'a str>) {
    let mut all_allergens = HashSet::<&'a str>::new();
    let mut all_ingredients = HashSet::<&'a str>::new();

    for i in ingredients.iter() {
        all_allergens.extend(&i.allergens);
        all_ingredients.extend(&i.ingredients);
    }

    (all_allergens, all_ingredients)
}

fn create_possible_allergens<'a>(ingredients: &'a Vec<Ingredients>, all_allergens: &HashSet<&'a str>, all_ingredients: &HashSet<&'a str>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut allergens = HashMap::<&'a str, HashSet<&'a str>>::new();

    for a in all_allergens {
        allergens.insert(a, all_ingredients.clone());
    }

    for i in ingredients.iter() {
        for a in i.allergens.iter() {
            if let Some(x) = allergens.get_mut(a) {
                x.retain(|y| i.ingredients.contains(y));
            }
        }
    }

    allergens
}

#[derive(Debug)]
struct Ingredients<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

lazy_static! {
    static ref INGREDIENTS_ALERGENS: Regex = Regex::new(r#"^([\w\s]+)(\(contains ([\w\s,]+)\))?$"#).unwrap();
}