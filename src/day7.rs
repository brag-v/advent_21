use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn task1(input: String) -> String {
    let positions: Vec<u32> = input.split(',').map(|num| num.parse().unwrap()).collect();

    // minimum position will be at one of the crab's positions
    let min_fuel: u32 = positions
        .iter()
        .map(|dest| positions.iter().map(|src| src.abs_diff(*dest)).sum())
        .min()
        .unwrap();

    min_fuel.to_string()
}

fn triangle_num(n: u32) -> u32 {
    n * (n + 1) / 2
}

pub fn task2(input: String) -> String {
    let positions: Vec<u32> = input.split(',').map(|num| num.parse().unwrap()).collect();
    let start = positions.iter().min().unwrap();
    let end = positions.iter().max().unwrap();

    let min_fuel: u32 = (*start..=*end)
        .into_par_iter()
        .map(|dest| {
            positions
                .iter()
                .map(|src| triangle_num(src.abs_diff(dest)))
                .sum()
        })
        .min()
        .unwrap();

    min_fuel.to_string()
}
