use std::fs;

pub fn day_eighteen() {
    let input = fs::read_to_string("input/day18.txt")
        .expect("oh no")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut sum = 0;
    for x in input.iter() {
        sum += eval(x);
    }

    println!("{}", sum);
}

fn eval(s: &str) -> i64 {
    eval_tokens(&lex(s)).0
}

fn eval_tokens(tokens: &[Token]) -> (i64, usize) {
    let mut op = Op::Plus;
    let mut result = 0;
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::Number(n) => match op {
                Op::Plus => result += n,
                Op::Multiply => result *= n,
            },
            Token::Op(o) => op = o,
            Token::OpenParen => {
                let n = eval_tokens(&tokens[i+1..tokens.len()]);
                i += n.1;
                match op {
                    Op::Plus => result += n.0,
                    Op::Multiply => result *= n.0,
                }
            },
            Token::CloseParen => return (result, i+1),
        }
        i += 1;
    }
    (result, tokens.len())
}

fn lex(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    loop {
        if let Some(c) = chars.next() {
            match c {
                ' ' => (),
                '+' => tokens.push(Token::Op(Op::Plus)),
                '*' => tokens.push(Token::Op(Op::Multiply)),
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                _ if c.is_digit(10) => {
                    let mut d = vec![c];
                    while let Some(&n) = chars.peek() {
                        if n.is_digit(10) {
                            d.push(n);
                        } else {
                            break;
                        }
                    }
                    let n: String = d.iter().collect();
                    tokens.push(Token::Number(n.parse().unwrap()))
                },
                _ => panic!("unexpected character {}", c),
            }
        } else {
            break;
        }
    }
    tokens
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Op(Op),
    Number(i64),
    OpenParen,
    CloseParen,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Plus,
    Multiply,
}
