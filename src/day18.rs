use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{fmt::Display, ops::Add};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct SnailPair {
    left: SnailNumber,
    right: SnailNumber,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum SnailNumber {
    Regular(u8),
    Pair(Box<SnailPair>),
}

impl Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNumber::Regular(num) => write!(f, "{num}"),
            SnailNumber::Pair(snail_pair) => {
                write!(f, "[")?;
                snail_pair.left.fmt(f)?;
                write!(f, ",")?;
                snail_pair.right.fmt(f)?;
                write!(f, "]")
            }
        }
    }
}

impl Add for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: Self) -> Self::Output {
        SnailNumber::Pair(Box::new(SnailPair {
            left: self,
            right: rhs,
        }))
        .reduce()
    }
}

#[derive(Debug)]
pub struct InvalidSnailNumber {}

impl TryFrom<&str> for SnailNumber {
    type Error = InvalidSnailNumber;

    fn try_from(value: &str) -> Result<Self, InvalidSnailNumber> {
        let mut number_stack = vec![];
        for symbol in value.bytes() {
            match symbol {
                // We don't check for matching parenthesis,
                // or that the pairs are comma separated.
                b'[' | b',' => (),
                b'0'..=b'9' => {
                    number_stack.push(SnailNumber::Regular(symbol - b'0'));
                }
                b']' => {
                    if let (Some(right), Some(left)) = (number_stack.pop(), number_stack.pop()) {
                        number_stack.push(SnailNumber::Pair(Box::new(SnailPair { left, right })));
                    } else {
                        return Err(InvalidSnailNumber {});
                    }
                }
                _ => return Err(InvalidSnailNumber {}),
            }
        }
        match number_stack.pop() {
            Some(top) => Ok(top),
            None => Err(InvalidSnailNumber {}),
        }
    }
}

impl SnailNumber {
    const fn magnitude(&self) -> u64 {
        match self {
            SnailNumber::Regular(num) => *num as u64,
            SnailNumber::Pair(pair) => 3 * pair.left.magnitude() + 2 * pair.right.magnitude(),
        }
    }

    const fn add_to_leftmost(&mut self, value: u8) {
        match self {
            SnailNumber::Regular(num) => *num += value,
            SnailNumber::Pair(snail_pair) => snail_pair.left.add_to_leftmost(value),
        }
    }

    const fn add_to_rightmost(&mut self, value: u8) {
        match self {
            SnailNumber::Regular(num) => *num += value,
            SnailNumber::Pair(snail_pair) => snail_pair.right.add_to_rightmost(value),
        }
    }

    fn split_big(&mut self, max_depth: u8) -> bool {
        match self {
            SnailNumber::Regular(num) => {
                if *num >= 10 {
                    *self = SnailNumber::Pair(Box::new(SnailPair {
                        left: SnailNumber::Regular(*num / 2),
                        right: SnailNumber::Regular(*num / 2 + *num % 2),
                    }));
                    max_depth == 0
                } else {
                    false
                }
            }
            SnailNumber::Pair(snail_pair) => {
                assert!(
                    max_depth > 0,
                    "Encountered snail number exceeding maximum nesting depth while trying to split big regular numbers"
                );
                // relies on short-circuiting of boolean expressions for correctness
                snail_pair.left.split_big(max_depth - 1)
                    || snail_pair.right.split_big(max_depth - 1)
            }
        }
    }

    fn explode_deep(&mut self, max_depth: u8) -> (u8, u8) {
        if let SnailNumber::Pair(pair) = self {
            if max_depth == 0 {
                if let (SnailNumber::Regular(left), SnailNumber::Regular(right)) =
                    (&pair.left, &pair.right)
                {
                    let explotion = (*left, *right);
                    *self = SnailNumber::Regular(0);
                    return explotion;
                }
                panic!("Unable to explode non-simple pair, {self}, nested below maximum depth");
            }
            let (outer_left, inner_right) = pair.left.explode_deep(max_depth - 1);
            if inner_right > 0 {
                pair.right.add_to_leftmost(inner_right);
            }
            let (inner_left, outer_right) = pair.right.explode_deep(max_depth - 1);
            if inner_left > 0 {
                pair.left.add_to_rightmost(inner_left);
            }
            (outer_left, outer_right)
        } else {
            (0, 0)
        }
    }

    fn reduce(mut self) -> Self {
        loop {
            self.explode_deep(4);
            if !self.split_big(4) {
                return self;
            }
        }
    }
}

pub fn task1(input: String) -> String {
    input
        .lines()
        .map(|line| SnailNumber::try_from(line).unwrap())
        .reduce(|acc, num| acc + num)
        .unwrap()
        .magnitude()
        .to_string()
}

pub fn task2(input: String) -> String {
    let snail_numbers: Vec<SnailNumber> = input
        .lines()
        .map(|line| SnailNumber::try_from(line).unwrap())
        .collect();

    snail_numbers
        .par_iter()
        .flat_map_iter(|left| {
            snail_numbers
                .iter()
                .map(|right| (left.clone() + right.clone()).magnitude())
        })
        .max()
        .unwrap()
        .to_string()
}
