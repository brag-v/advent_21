use crate::grid::Coord;
use std::fmt::Debug;

fn parse_map<I, E>(input: I) -> Vec<Vec<u8>>
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    input
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|height| height as u8 - b'0')
                .collect()
        })
        .collect()
}

fn find_low_points(map: &[Vec<u8>]) -> Vec<Coord> {
    let width = map[0].len();
    let height = map.len();
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(move |(x, num)| {
                    (Coord { x: *x, y })
                        .adjacent(width, height)
                        .iter()
                        .all(|adj| map[adj.y][adj.x] > **num)
                })
                .map(move |(x, _height)| Coord {x, y})
        })
        .collect()
}

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let map = parse_map(input);
    find_low_points(&map)
        .iter()
        .map(|pos| u64::from(map[pos.y][pos.x] + 1))
        .sum::<u64>()
        .to_string()
}

#[derive(Clone, Copy)]
enum BasinTile {
    None,
    Basin,
    Source,
}

/// if pos marks a basin source, mark the corresponding basin and return it's size
fn mark_basin(pos: Coord, basin_map: &mut [Vec<BasinTile>], height_map: &[Vec<u8>]) -> u64 {
    let mut size = 0;
    if matches!(basin_map[pos.y][pos.x], BasinTile::Source) {
        let mut coords = vec![pos];
        size += 1;
        while let Some(pos) = coords.pop() {
            for adj in pos.adjacent(basin_map[0].len(), basin_map.len()) {
                if height_map[adj.y][adj.x] < 9
                    && matches!(basin_map[adj.y][adj.x], BasinTile::None)
                    && height_map[adj.y][adj.x] > height_map[pos.y][pos.x]
                {
                    basin_map[adj.y][adj.x] = BasinTile::Basin;
                    coords.push(adj);
                    size += 1;
                }
            }
        }
    }
    size
}

/// Marks all basins and returns the size of the three largest
fn top_three_basins(basin_map: &mut [Vec<BasinTile>], height_map: &[Vec<u8>]) -> [u64; 3] {
    let mut top_three = [0; 3];
    for y in 0..basin_map.len() {
        for x in 0..basin_map[0].len() {
            let size = mark_basin(Coord { x, y }, basin_map, height_map);
            if size > top_three[0] {
                top_three[0] = size;
                top_three.sort_unstable();
            }
        }
    }
    top_three
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let height_map = parse_map(input);
    let mut basin_map = vec![vec![BasinTile::None; height_map[0].len()]; height_map.len()];
    for src in find_low_points(&height_map) {
        basin_map[src.y][src.x] = BasinTile::Source;
    }
    let top_three = top_three_basins(&mut basin_map, &height_map);
    let prod = top_three[0] * top_three[1] * top_three[2];
    prod.to_string()
}
