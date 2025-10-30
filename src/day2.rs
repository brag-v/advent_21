use std::fmt::Debug;

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for line in input.map(|line| line.unwrap()) {
        let (dir, num) = line.split_once(' ').unwrap();
        let num: i32 = num.parse().unwrap();
        match dir.chars().next() {
            Some('f') => x += num,
            Some('u') => y -= num,
            Some('d') => y += num,
            _ => panic!("Invalid direction"),
        }
    }

    (x.abs() * y.abs()).to_string()
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;
    for line in input.map(|line| line.unwrap()) {
        let (dir, num) = line.split_once(' ').unwrap();
        let num: i32 = num.parse().unwrap();
        match dir.chars().next() {
            Some('f') => {
                x += num;
                y += num * aim;
            }
            Some('u') => aim -= num,
            Some('d') => aim += num,
            _ => panic!("Invalid direction"),
        }
    }

    (x.abs() * y.abs()).to_string()
}
