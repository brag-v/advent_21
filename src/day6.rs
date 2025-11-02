use std::{collections::HashMap, fmt::Debug};

fn total_number_of_fish(counter: u8, time: u16, memo: &mut HashMap<(u8, u16), u64>) -> u64 {
    if time == 0 {
        return 1;
    }
    if let Some(count) = memo.get(&(counter, time)) {
        return *count;
    }
    let count = match counter {
        0 => total_number_of_fish(6, time - 1, memo) + total_number_of_fish(8, time - 1, memo),
        1.. => total_number_of_fish(counter - 1, time - 1, memo),
    };
    memo.insert((counter, time), count);
    count
}

fn solve_tasks<I, E>(mut input: I, time: u16) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let mut memo = HashMap::new();
    input
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .map(|num| total_number_of_fish(num, time, &mut memo))
        .sum::<u64>()
        .to_string()
}

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    solve_tasks(input, 80)
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    solve_tasks(input, 256)
}
