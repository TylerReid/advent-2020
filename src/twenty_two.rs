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

    let mut p1_part2 = player_one.clone();
    let mut p2_part2 = player_two.clone();

    while player_one.len() > 0 && player_two.len() > 0 {
        play_hand(&mut player_one, &mut player_two);
    }

    let mut winner = if player_one.len() == 0 { player_two } else { player_one };

    println!("winning score: {}", winning_score(&mut winner));

    play_recursive(&mut Vec::new(), &mut p1_part2, &mut p2_part2);
}

fn play_recursive(previous_hands: &mut Vec<(VecDeque<u32>, VecDeque<u32>)>, p1: &mut VecDeque<u32>, p2: &mut VecDeque<u32>) -> bool {
    loop {
        if previous_hands.contains(&(p1.clone(), p2.clone())) {
            return true;
        }

        previous_hands.push((p1.clone(), p2.clone()));

        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();
        if p1_card == p2_card {
            panic!("did not expect two cards to be the same {}", p1_card);
        }

        if p1_card <= p1.len() as u32 && p2_card <= p2.len() as u32 {
            let mut p1_copy = p1.clone();
            let mut p2_copy = p2.clone();
            while p1_copy.len() > p1_card as usize {
                p1_copy.pop_back();
            }
            while p2_copy.len() > p2_card as usize {
                p2_copy.pop_back();
            }
            let p1_wins = play_recursive(&mut Vec::new(), &mut p1_copy, &mut p2_copy);

            if p1_wins {
                give_winner_cards(p1, p1_card, p2_card);
            } else {
                give_winner_cards(p2, p2_card, p1_card);
            }
        } else if p1_card > p2_card {
            give_winner_cards(p1, p1_card, p2_card);
        } else {
            give_winner_cards(p2, p2_card, p1_card);
        }

        if p1.len() == 0 {
            println!("{}", winning_score(p2));
            return false;
        }

        if p2.len() == 0 {
            println!("{}", winning_score(p1));
            return true;
        }
    }
}

fn play_hand(p1: &mut VecDeque<u32>, p2: &mut VecDeque<u32>) {
    let p1_card = p1.pop_front().unwrap();
    let p2_card = p2.pop_front().unwrap();
    if p1_card == p2_card {
        panic!("did not expect two cards to be the same {}", p1_card);
    }
    if p1_card > p2_card {
        give_winner_cards(p1, p1_card, p2_card);
    } else {
        give_winner_cards(p2, p2_card, p1_card);
    }
}

fn give_winner_cards(p: &mut VecDeque<u32>, win: u32, lose: u32) {
    p.push_back(win);
    p.push_back(lose);
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
