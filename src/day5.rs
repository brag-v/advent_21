use std::{
    cmp::{Ordering, max},
    fmt::Debug,
};

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

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

fn parse_coords(coords: &str) -> Coord {
    let (x, y) = coords.split_once(',').unwrap();
    Coord {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}

fn get_coords<I>(input: I) -> impl Iterator<Item = Line>
where
    I: Iterator<Item = String>,
{
    input.map(|line| {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();
        Line {
            start: parse_coords(lhs),
            end: parse_coords(rhs),
        }
    })
}

fn find_map_dimention(lines: &[Line]) -> (usize, usize) {
    let height = lines
        .iter()
        .map(|line| max(line.start.y, line.end.y))
        .max()
        .unwrap()
        + 1;
    let width = lines
        .iter()
        .map(|line| max(line.start.x, line.end.x))
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
        .map(|row| row.iter().filter(|line_count| **line_count > 1).count())
        .sum()
}

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let lines: Vec<Line> = get_coords(input.map(|line| line.unwrap()))
        .filter(Line::is_straight)
        .collect();
    let (width, height) = find_map_dimention(&lines);
    let mut map: Vec<Vec<u8>> = vec![vec![0; width]; height];
    for line in lines {
        update_map(&mut map, &line);
    }
    count_overlaps(&map).to_string()
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let lines: Vec<Line> = get_coords(input.map(|line| line.unwrap())).collect();
    let (width, height) = find_map_dimention(&lines);
    let mut map: Vec<Vec<u8>> = vec![vec![0; width]; height];
    for line in lines {
        update_map(&mut map, &line);
    }
    count_overlaps(&map).to_string()
}
