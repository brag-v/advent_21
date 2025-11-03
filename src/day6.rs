use std::fmt::Debug;

const NEW_FISH_COUNTER: usize = 8;
const RESET_CONTER: usize = 6;

fn to_index(counter: usize, time: usize) -> usize {
    (time - 1) * NEW_FISH_COUNTER + counter
}

fn total_number_of_fish(counter: usize, time: usize, memo: &mut [Option<u64>]) -> u64 {
    if time == 0 {
        return 1;
    }
    if let Some(count) = memo[to_index(counter, time)] {
        return count;
    }
    let count = match counter {
        0 => {
            total_number_of_fish(RESET_CONTER, time - 1, memo)
                + total_number_of_fish(NEW_FISH_COUNTER, time - 1, memo)
        }
        1.. => total_number_of_fish(counter - 1, time - 1, memo),
    };
    memo[to_index(counter, time)] = Some(count);
    count
}

fn solve_tasks<I, E>(mut input: I, time: usize) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let mut memo = vec![None; NEW_FISH_COUNTER * time];
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
