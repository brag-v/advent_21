use crate::grid::Coord;
use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    /// Check if line is perfectly vertical or horizontal
    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    /// Returns the ordered coordinates on the line.
    /// Line must be vertical, horizontal, or 45 degrees
    fn interpolate(&self) -> Vec<Coord> {
        let length =
            (self.start.x.abs_diff(self.end.x) + 1).max(self.start.y.abs_diff(self.end.y) + 1);

        let x_vec: Vec<usize> = match self.start.x.cmp(&self.end.x) {
            Ordering::Less => (self.start.x..=self.end.x).collect(),
            Ordering::Equal => vec![self.start.x; length],
            Ordering::Greater => (self.end.x..=self.start.x).rev().collect(),
        };
        let y_vec: Vec<usize> = match self.start.y.cmp(&self.end.y) {
            Ordering::Less => (self.start.y..=self.end.y).collect(),
            Ordering::Equal => vec![self.start.y; length],
            Ordering::Greater => (self.end.y..=self.start.y).rev().collect(),
        };

        x_vec
            .iter()
            .zip(y_vec.iter())
            .map(|(x, y)| Coord { x: *x, y: *y })
            .collect()
    }
}

fn get_lines(input: &str) -> impl Iterator<Item = Line> {
    input.lines().map(|line| {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();
        Line {
            start: Coord::from(lhs),
            end: Coord::from(rhs),
        }
    })
}

/// Finds the minimum map dimentions based on line endpoint coords
fn find_map_dimention(lines: &[Line]) -> (usize, usize) {
    let height = lines
        .iter()
        .map(|line| line.start.y.max(line.end.y))
        .max()
        .unwrap()
        + 1;
    let width = lines
        .iter()
        .map(|line| line.start.x.max(line.end.x))
        .max()
        .unwrap()
        + 1;
    (width, height)
}

fn update_map(map: &mut [Vec<u8>], straight_line: &Line) {
    for coord in straight_line.interpolate() {
        map[coord.y][coord.x] += 1;
    }
}

fn count_overlaps(map: &[Vec<u8>]) -> usize {
    map.iter()
        .flat_map(|row| row.iter().filter(|line_count| **line_count > 1))
        .count()
}

pub fn task1(input: String) -> String {
    let lines: Vec<Line> = get_lines(&input).filter(Line::is_straight).collect();
    let (width, height) = find_map_dimention(&lines);
    let mut map: Vec<Vec<u8>> = vec![vec![0; width]; height];
    for line in lines {
        update_map(&mut map, &line);
    }
    count_overlaps(&map).to_string()
}

pub fn task2(input: String) -> String {
    let lines: Vec<Line> = get_lines(&input).collect();
    let (width, height) = find_map_dimention(&lines);
    let mut map: Vec<Vec<u8>> = vec![vec![0; width]; height];
    for line in lines {
        update_map(&mut map, &line);
    }
    count_overlaps(&map).to_string()
}
