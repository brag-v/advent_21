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

/// return a list of adjecent coordinates within the bounds (0..width) and (0..height)
fn adjecent(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut a = Vec::with_capacity(4);
    if x != 0 {
        a.push((x - 1, y));
    }
    if y != 0 {
        a.push((x, y - 1));
    }
    if x != width - 1 {
        a.push((x + 1, y));
    }
    if y != height - 1 {
        a.push((x, y + 1));
    }
    a
}

fn find_low_points(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let width = map[0].len();
    let height = map.len();
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(move |(x, num)| {
                    adjecent(*x, y, width, height)
                        .iter()
                        .all(|(adj_x, adj_y)| map[*adj_y][*adj_x] > **num)
                })
                .map(move |(x, _height)| (x, y))
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
        .map(|(x, y)| u64::from(map[*y][*x] + 1))
        .sum::<u64>()
        .to_string()
}

#[derive(Clone, Copy)]
enum BasinTile {
    None,
    Basin,
    Source,
}

/// if (x, y) marks a basin source, mark the corresponding basin and return it's size
fn mark_basin(x: usize, y: usize, basin_map: &mut [Vec<BasinTile>], height_map: &[Vec<u8>]) -> u64 {
    let mut size = 0;
    if matches!(basin_map[y][x], BasinTile::Source) {
        let mut coords = vec![(x, y)];
        size += 1;
        while let Some((new_x, new_y)) = coords.pop() {
            for (adj_x, adj_y) in adjecent(new_x, new_y, basin_map[0].len(), basin_map.len()) {
                if height_map[adj_y][adj_x] < 9
                    && matches!(basin_map[adj_y][adj_x], BasinTile::None)
                    && height_map[adj_y][adj_x] > height_map[new_y][new_x]
                {
                    basin_map[adj_y][adj_x] = BasinTile::Basin;
                    coords.push((adj_x, adj_y));
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
            let size = mark_basin(x, y, basin_map, height_map);
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
    for (src_x, src_y) in find_low_points(&height_map) {
        basin_map[src_y][src_x] = BasinTile::Source;
    }
    let top_three = top_three_basins(&mut basin_map, &height_map);
    let prod = top_three[0] * top_three[1] * top_three[2];
    prod.to_string()
}
