use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[derive(Debug)]
struct Cave<'c> {
    big: bool,
    neighbors: Vec<&'c str>,
}

fn is_big_cave(cave_id: &str) -> bool {
    cave_id.chars().next().is_some_and(char::is_uppercase)
}

fn parse_cave_system(input: &[String]) -> HashMap<&str, Cave<'_>> {
    let mut caves = HashMap::new();
    for (left_id, right_id) in input.iter().map(|link| link.split_once('-').unwrap()) {
        caves
            .entry(left_id)
            .and_modify(|cave: &mut Cave| cave.neighbors.push(right_id))
            .or_insert(Cave {
                big: is_big_cave(left_id),
                neighbors: vec![&right_id],
            });
        caves
            .entry(right_id)
            .and_modify(|cave: &mut Cave| cave.neighbors.push(left_id))
            .or_insert(Cave {
                big: is_big_cave(right_id),
                neighbors: vec![&left_id],
            });
    }
    caves
}

fn count_paths<'a>(
    from_id: &'a str,
    cave_system: &'a HashMap<&str, Cave>,
    visited: &mut HashSet<&'a str>,
) -> u64 {
    if from_id == "end" {
        return 1;
    }
    let cave = cave_system.get(from_id).unwrap();
    if !cave.big && visited.contains(from_id) {
        return 0;
    }
    visited.insert(from_id);

    let path_count = cave
        .neighbors
        .iter()
        .map(|neighbor_id| count_paths(neighbor_id, cave_system, visited))
        .sum();

    visited.remove(from_id);
    path_count
}

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let links = input.map(|line| line.unwrap()).collect::<Vec<String>>();
    let cave_system = parse_cave_system(&links);
    count_paths("start", &cave_system, &mut HashSet::new()).to_string()
}

fn count_paths_with_duplicates<'a>(
    from_id: &'a str,
    cave_system: &'a HashMap<&str, Cave>,
    visits: &mut HashMap<&'a str, u8>,
    mut remaining_revisits: u8,
) -> u64 {
    if from_id == "end" {
        return 1;
    }
    let cave = cave_system.get(from_id).unwrap();
    if !cave.big {
        let visits = visits.entry(from_id).or_insert(0);
        if *visits != 0 {
            if remaining_revisits == 0 || from_id == "start" {
                return 0;
            }
            remaining_revisits -= 1;
        }
        *visits += 1;
    }

    let path_count = cave
        .neighbors
        .iter()
        .map(|neighbor_id| {
            count_paths_with_duplicates(neighbor_id, cave_system, visits, remaining_revisits)
        })
        .sum();

    if !cave.big {
        *visits.get_mut(from_id).unwrap() -= 1;
    }
    path_count
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let links = input.map(|line| line.unwrap()).collect::<Vec<String>>();
    let cave_system = parse_cave_system(&links);
    count_paths_with_duplicates("start", &cave_system, &mut HashMap::new(), 1).to_string()
}
