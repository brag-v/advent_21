use crate::grid::Coord;
use std::{cmp::Ordering, fmt::Debug, usize};

#[derive(Debug)]
enum Orientation {
    Vertical,
    Horizontal,
}
use Orientation::{Horizontal, Vertical};

#[derive(Debug)]
struct Fold {
    axis: Orientation,
    distance: usize,
}

fn get_coords_and_folds(input: &str) -> (Vec<Coord>, Vec<Fold>) {
    let (coord_input, fold_input) = input.split_once("\n\n").unwrap();
    let coords = coord_input.split("\n").map(Coord::from).collect();
    let folds = fold_input
        .split("\n")
        .map(|line| {
            line.strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap()
        })
        .map(|(axis, distance)| Fold {
            axis: match axis {
                "x" => Vertical,
                "y" => Horizontal,
                _ => panic!(),
            },
            distance: distance.parse().unwrap(),
        })
        .collect();
    (coords, folds)
}

fn fold_coord(fold: &Fold, coord: &mut Coord) {
    match fold.axis {
        Vertical => {
            if coord.x > fold.distance {
                coord.x = 2 * fold.distance - coord.x
            }
        }
        Horizontal => {
            if coord.y > fold.distance {
                coord.y = 2 * fold.distance - coord.y
            }
        }
    }
}

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let input = input
        .map(|line| line.unwrap())
        .fold(String::new(), |acc, line| acc + &line + "\n")
        .trim_end()
        .to_owned();
    let (mut coords, folds) = get_coords_and_folds(&input);

    for coord in coords.iter_mut() {
        fold_coord(&folds[0], coord);
    }

    coords.sort_unstable_by(|a, b| match a.x.cmp(&b.x) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.y.cmp(&b.y),
        Ordering::Greater => Ordering::Greater,
    });
    coords.dedup();
    coords.len().to_string()
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let input = input
        .map(|line| line.unwrap())
        .fold(String::new(), |acc, line| acc + &line + "\n")
        .trim_end()
        .to_owned();
    let (mut coords, folds) = get_coords_and_folds(&input);

    let mut width = usize::MAX;
    let mut height = usize::MAX;
    for fold in folds {
        for coord in coords.iter_mut() {
            fold_coord(&fold, coord);
        }
        match fold.axis {
            Vertical => width = fold.distance,
            Horizontal => height = fold.distance,
        }
    }

    let mut code_print = vec![vec![false; width]; height];
    for coord in coords {
        code_print[coord.y][coord.x] = true;
    }

    code_print
        .iter()
        .map(|line| {
            line.iter()
                .map(|point| if *point { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
