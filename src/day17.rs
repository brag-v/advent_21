use regex::Regex;

#[derive(Debug)]
struct Area {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

fn parse_target_area(input: &str) -> Area {
    let re = Regex::new(
        r"target area: x=(?<left>-?\d*)..(?<right>-?\d*), y=(?<bottom>-?\d*)..(?<top>-?\d*)",
    )
    .unwrap();
    let caps = re.captures(input).unwrap();
    Area {
        left: caps["left"].parse().unwrap(),
        right: caps["right"].parse().unwrap(),
        top: caps["top"].parse().unwrap(),
        bottom: caps["bottom"].parse().unwrap(),
    }
}

const fn triangle_number(number: i32) -> i32 {
    number * (number + 1) / 2
}

fn highest_hit(target_area: Area) -> Option<i32> {
    // the highest y value is reached when the probe exactly hits the bottom or the top
    // of the target area on it's way down.
    // If the target area contains y = 0, arbitrary hights are reachable
    // We assume that the target area is always reachable in the x direction
    if 0 >= target_area.bottom && 0 <= target_area.top {
        return None;
    }
    if target_area.bottom < 0 {
        Some(triangle_number(-target_area.bottom - 1))
    } else {
        Some(triangle_number(target_area.top))
    }
}

pub fn task1(input: String) -> String {
    let target_area = parse_target_area(&input);
    highest_hit(target_area).unwrap().to_string()
}

pub fn task2(input: String) -> String {
    todo!("Task not implemented");
}
