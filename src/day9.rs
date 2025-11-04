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
                .map(|height| height as u8 - '0' as u8)
                .collect()
        })
        .collect()
}

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
                // .inspect(move |(x, num)| println!("({x}, {y}), {num}"))
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
        .map(|(x, y)| (map[*y][*x] + 1) as u64)
        .sum::<u64>()
        .to_string()
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    todo!("Task not implemented");
}
