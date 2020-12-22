use std::fs;
use std::collections::VecDeque;

pub fn f() {
    let input = fs::read_to_string("input/day22.txt")
        .expect("oh no");

    let mut player_one = VecDeque::<u32>::new();
    let mut player_two = VecDeque::<u32>::new();
    let mut on_player_one_setup = true;
    for line in input.lines() {
        if line == "Player 1:" || line == "" {
            continue;
        }
        if line == "Player 2:" {
            on_player_one_setup = false;
            continue;
        }
        if on_player_one_setup {
            player_one.push_back(line.parse().unwrap());
        } else {
            player_two.push_back(line.parse().unwrap());
        }
    }

    println!("p1 {:#?}", player_one);
    println!("p2 {:#?}", player_two);

    while player_one.len() > 0 && player_two.len() > 0 {
        play_hand(&mut player_one, &mut player_two);
    }
    
    
    println!("p1 {:#?}", player_one);
    println!("p2 {:#?}", player_two);

    let mut winner = if player_one.len() == 0 { player_two } else { player_one };

    println!("winning score: {}", winning_score(&mut winner));
}

fn play_hand(p1: &mut VecDeque<u32>, p2: &mut VecDeque<u32>) {
    let p1_card = p1.pop_front().unwrap();
    let p2_card = p2.pop_front().unwrap();
    if p1_card == p2_card {
        panic!("did not expect two cards to be the same {}", p1_card);
    }
    if p1_card > p2_card {
        p1.push_back(p1_card);
        p1.push_back(p2_card);
    } else {
        p2.push_back(p2_card);
        p2.push_back(p1_card);
    }
}

fn winning_score(p: &mut VecDeque<u32>) -> u32 {
    let mut multiplier = 1;
    let mut score = 0;
    while p.len() > 0 {
        let c = p.pop_back().unwrap();
        score += multiplier * c;
        multiplier += 1;
    }
    score
}
