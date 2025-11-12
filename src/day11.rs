use crate::grid::Coord;

#[derive(Clone, Copy)]
enum Octopus {
    Charging(u8),
    Flashing,
}

fn parse_map(input: String) -> Vec<Vec<Octopus>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|counter| Octopus::Charging(counter as u8 - b'0'))
                .collect()
        })
        .collect()
}

fn charge_and_flash(pos: Coord, map: &mut [Vec<Octopus>]) {
    let mut to_charge = vec![pos];
    while let Some(pos) = to_charge.pop() {
        let octopus = &mut map[pos.y][pos.x];
        if let Octopus::Charging(charge) = *octopus {
            if charge == 9 {
                *octopus = Octopus::Flashing;
                to_charge.append(&mut pos.adjecent_with_diagonals(map[0].len(), map.len()));
            } else {
                *octopus = Octopus::Charging(charge + 1);
            }
        }
    }
}

fn time_step(map: &mut [Vec<Octopus>]) -> u64 {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            charge_and_flash(Coord { x, y }, map);
        }
    }
    // reset and count flashes
    let mut flash_count = 0;
    for row in map.iter_mut() {
        for octopus in row.iter_mut() {
            if matches!(octopus, Octopus::Flashing) {
                *octopus = Octopus::Charging(0);
                flash_count += 1;
            }
        }
    }
    flash_count
}

pub fn task1(input: String) -> String {
    let mut map = parse_map(input);
    let mut flash_count = 0;
    for _time_step in 0..100 {
        flash_count += time_step(&mut map);
    }
    flash_count.to_string()
}

pub fn task2(input: String) -> String {
    let mut map = parse_map(input);
    let desired_flash_count = (map.len() * map[0].len()) as u64;
    let mut i = 0;
    loop {
        i += 1;
        if desired_flash_count == time_step(&mut map) {
            return i.to_string();
        }
    }
}
