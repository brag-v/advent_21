use std::{cmp::Reverse, collections::BinaryHeap};

use crate::grid::{Coord, Map};

#[derive(Clone, Copy, Eq, PartialEq)]
struct PathNode {
    pos: Coord,
    cost: u32,
    heuristic: u32,
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cost + self.heuristic).cmp(&(other.cost + other.heuristic))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// find the riks costs of the shortest path diagonally across the risk map using the A* algorithm
fn min_path_cost(map: &Map<u8>) -> u32 {
    let mut costs = Map::filled_with(None, map.width(), map.height());
    let goal = Coord {
        x: map.width() - 1,
        y: map.height() - 1,
    };
    let start = Coord { x: 0, y: 0 };
    let start_node = PathNode {
        pos: start,
        cost: 0,
        heuristic: start.manhattan_distance(&goal) as u32,
    };
    costs[start] = Some(start_node);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(start_node));
    while let Some(node) = queue.pop().map(|node| node.0) {
        if node.pos == goal {
            break;
        }
        if costs[node.pos].is_some_and(|n| n.cost < node.cost) {
            continue;
        }
        for adj in node.pos.adjacent(map.width(), map.height()) {
            let adj_node = costs[adj];
            if adj_node.is_some_and(|adj_node| adj_node.cost > node.cost + map[adj] as u32)
                || adj_node.is_none()
            {
                let new_adj_node = PathNode {
                    pos: adj,
                    cost: map[adj] as u32 + node.cost,
                    heuristic: adj.manhattan_distance(&goal) as u32,
                };
                costs[adj] = Some(new_adj_node);
                queue.push(Reverse(new_adj_node));
            }
        }
    }

    costs[goal].unwrap().cost
}

pub fn task1(input: String) -> String {
    let map = Map::try_from_str(&input, |num| num.to_digit(10).unwrap() as u8).unwrap();
    min_path_cost(&map).to_string()
}

pub fn task2(input: String) -> String {
    let growth = 5;
    let small_map = Map::try_from_str(&input, |num| num.to_digit(10).unwrap() as u8).unwrap();
    let mut big_map = vec![0; small_map.height() * small_map.width() * growth * growth];
    for (y, row) in small_map.rows().enumerate() {
        for (x, risk) in row.iter().enumerate() {
            for big_y in 0..growth {
                for big_x in 0..growth {
                    big_map[(y + big_y * small_map.height()) * small_map.width() * growth
                        + x
                        + big_x * small_map.width()] =
                        (risk + big_x as u8 + big_y as u8 - 1) % 9 + 1;
                }
            }
        }
    }

    let big_map = Map::new(
        big_map.into_boxed_slice(),
        growth * small_map.width(),
        growth * small_map.height(),
    );
    min_path_cost(&big_map).to_string()
}
