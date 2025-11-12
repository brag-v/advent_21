use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Paren {
    Normal,
    Square,
    Curly,
    Pointy,
}

enum ParenParseResult {
    RemainingStack(Vec<Paren>),
    InvalidParen(Paren),
}

fn score_wrong_paren(paren: Paren) -> u64 {
    match paren {
        Paren::Normal => 3,
        Paren::Square => 57,
        Paren::Curly => 1197,
        Paren::Pointy => 25137,
    }
}

fn score_missing_paren(paren: Paren) -> u64 {
    match paren {
        Paren::Normal => 1,
        Paren::Square => 2,
        Paren::Curly => 3,
        Paren::Pointy => 4,
    }
}

fn checked_pop(paren: char, paren_stack: &mut Vec<Paren>) -> Option<Paren> {
    let right_paren = match paren {
        ')' => Paren::Normal,
        ']' => Paren::Square,
        '}' => Paren::Curly,
        '>' => Paren::Pointy,
        _ => panic!(),
    };
    match paren_stack.pop() {
        None => Some(right_paren),
        Some(left_paren) => {
            if left_paren == right_paren {
                None
            } else {
                Some(right_paren)
            }
        }
    }
}

fn parse_parens(parens: &str) -> ParenParseResult {
    let mut paren_stack = vec![];
    // TODO: maybe just make this a regular loop
    if let Some(mistake) = parens.chars().find_map(|paren| match paren {
        '(' => {
            paren_stack.push(Paren::Normal);
            None
        }
        '[' => {
            paren_stack.push(Paren::Square);
            None
        }
        '{' => {
            paren_stack.push(Paren::Curly);
            None
        }
        '<' => {
            paren_stack.push(Paren::Pointy);
            None
        }
        ')' | ']' | '}' | '>' => checked_pop(paren, &mut paren_stack),
        _ => panic!(),
    }) {
        ParenParseResult::InvalidParen(mistake)
    } else {
        ParenParseResult::RemainingStack(paren_stack)
    }
}

pub fn task1(input: String) -> String {
    input
        .lines()
        .filter_map(|line| {
            if let ParenParseResult::InvalidParen(paren) = parse_parens(line) {
                Some(score_wrong_paren(paren))
            } else {
                None
            }
        })
        .sum::<u64>()
        .to_string()
}

pub fn task2(input: String) -> String {
    let mut missing_paren_scores: Vec<u64> = input
        .lines()
        .filter_map(|line| {
            if let ParenParseResult::RemainingStack(paren_stack) = parse_parens(line) {
                Some(
                    paren_stack
                        .iter()
                        .rev()
                        .fold(0, |acc, paren| 5 * acc + score_missing_paren(*paren)),
                )
            } else {
                None
            }
        })
        .collect();
    let n = missing_paren_scores.len();
    missing_paren_scores
        .select_nth_unstable(n / 2)
        .1
        .to_string()
}
