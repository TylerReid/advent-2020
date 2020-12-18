use std::fs;
use std::collections::VecDeque;

pub fn day_eighteen() {
    let input = fs::read_to_string("input/day18.txt")
        .expect("oh no")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut sum = 0;
    for x in input.iter() {
        sum += eval(&mut parse(&lex(x)));
    }

    println!("{}", sum);
}

fn eval(t: &mut VecDeque<Token>) -> i64 {
    let mut stack = Vec::new();
    while let Some(token) = t.pop_front() {
        match token {
            Token::Number(n) => stack.push(n),
            Token::Op(o) => {
                let l = stack.pop().unwrap();
                let r = stack.pop().unwrap();
                match o {
                    Op::Plus => stack.push(l + r),
                    Op::Multiply => stack.push(l * r),
                    _ => panic!("unexpected op {:?}", o),
                }
            }
        }
    }
    stack.pop().unwrap()
}

fn lex(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            ' ' => (),
            '+' => tokens.push(Token::Op(Op::Plus)),
            '*' => tokens.push(Token::Op(Op::Multiply)),
            '(' => tokens.push(Token::Op(Op::OpenParen)),
            ')' => tokens.push(Token::Op(Op::CloseParen)),
            _ if c.is_digit(10) => {
                let mut d = vec![c];
                while let Some(&n) = chars.peek() {
                    if n.is_digit(10) {
                        d.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let n: String = d.iter().collect();
                tokens.push(Token::Number(n.parse().unwrap()))
            },
            _ => panic!("unexpected character {}", c),
        }
    }
    tokens
}

fn parse(t: &[Token]) -> VecDeque<Token> {
    let mut tokens = t.iter().peekable();
    let mut op_stack = Vec::<Op>::new();
    let mut output = VecDeque::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::Number(_) => output.push_back(*token),
            Token::Op(op) => {
                match op {
                    Op::OpenParen => op_stack.push(*op),
                    Op::Plus | Op::Multiply => {
                        while let Some(top_op) = op_stack.last() {
                            if let Op::OpenParen = top_op {
                                break;
                            } else if top_op.precedence() >= op.precedence() {
                                output.push_back(Token::Op(op_stack.pop().unwrap()));
                            } else {
                                break;
                            }
                        }
                        op_stack.push(*op);
                    },
                    Op::CloseParen => {
                        while let Some(top_op) = op_stack.last() {
                            if let Op::OpenParen = top_op {
                                op_stack.pop();
                                break;
                            } else {
                                output.push_back(Token::Op(op_stack.pop().unwrap()));
                            }
                        }
                    },
                }
            }
        }
    }

    while let Some(_) = op_stack.last() {
        output.push_back(Token::Op(op_stack.pop().unwrap()));
    }
    output
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Op(Op),
    Number(i64),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Plus,
    Multiply,
    OpenParen,
    CloseParen,
}

impl Op {
    fn precedence(&self) -> u8 {
        match self {
            Op::Multiply => 1,
            Op::Plus => 2,
            Op::OpenParen => 42,
            Op::CloseParen => 42,
        }
    }
}
