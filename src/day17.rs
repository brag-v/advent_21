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
        r"target area: x=(?<left>-?\d*)\.\.(?<right>-?\d*), y=(?<bottom>-?\d*)\.\.(?<top>-?\d*)",
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

const fn highest_hit(target_area: &Area) -> Option<i32> {
    // the highest y value is reached when the probe exactly hits the bottom or the top
    // of the target area on it's way down.
    // If the target area contains y = 0, arbitrary hights are reachable
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
    highest_hit(&target_area).unwrap().to_string()
}

#[allow(dead_code)] // TODO: work smarter not harder
fn get_x_velocities(target_area: &Area) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut valid_velocities = Vec::new();
    let mut stops = Vec::new();
    for initial_velocity in 0..target_area.right {
        let mut pos = 0;
        print!("{initial_velocity}:");
        for (time, vel) in (1..=initial_velocity).rev().enumerate() {
            pos += vel;
            if ((target_area.left)..=(target_area.right)).contains(&pos) {
                while valid_velocities.len() < time + 2 {
                    valid_velocities.push(vec![initial_velocity]);
                }
                valid_velocities[time + 1].push(initial_velocity);
            }
        }
        if (target_area.left..=target_area.right).contains(&pos) {
            stops.push(initial_velocity);
        }
        println!();
        println!("{valid_velocities:?}");
    }
    (valid_velocities, stops)
}

#[allow(dead_code)] // TODO: work smarter not harder
fn get_y_velocities(target_area: &Area) -> Vec<usize> {
    if target_area.top > 0 {
        // in both test and task input, the target area is below 0,0
        todo!()
    }

    let mut time_counts = Vec::new();
    for initial_velocity in target_area.bottom..0 {
        let mut pos = 0;
        let mut vel = initial_velocity;
        let mut direct_time = 0;
        print!("{initial_velocity}, {}:", -initial_velocity - 1);
        while pos > target_area.bottom {
            pos += vel;
            vel -= 1;
            direct_time += 1;
            if ((target_area.bottom)..=(target_area.top)).contains(&pos) {
                // While direct time shoots down to the target area, indirect time shoots
                // upwards before reaching back down.
                // This includes the 0 as initial velocity which reaches -1 velocity
                let indirect_time = (-initial_velocity as usize - 1) * 2 + 1 + direct_time;
                for time in [indirect_time, direct_time] {
                    if time_counts.len() < time + 1 {
                        time_counts.resize(time + 1, 0);
                    }
                    time_counts[time] += 1;
                    print!(" {time}");
                }
            }
        }
        println!();
        println!("{time_counts:?}");
    }
    time_counts
}

pub fn task2(input: String) -> String {
    let target_area = parse_target_area(&input);

    // TODO: figure out how to work smarter
    // method kind of worked at some point: it found all possible ways to reach inside the target
    // area at a point in time. This did, however, double count initial velocities that stays
    // inside the target area for multiple time steps, and there were no easy way to eliminate
    // these double countings
    // let (x_counts, stop_count) = get_x_velocities(&target_area);
    // println!("{x_counts:?}, {stop_count}");
    // let y_counts = get_y_velocities(&target_area);
    // println!("{y_counts:?}");
    // y_counts
    //     .iter()
    //     .zip_longest(x_counts.iter())
    //     .map(|counts| match counts {
    //         EitherOrBoth::Both(y_count, x_count) => y_count * x_count,
    //         EitherOrBoth::Left(y_count) => y_count * stop_count,
    //         EitherOrBoth::Right(_) => 0,
    //     })
    //     .sum::<usize>()
    //     .to_string()

    // work harder:
    // assume target area below y = 0
    // try all velocities in the box:
    // 0,-(bottom - 1)  .. right,-(bottom - 1)
    //      ..          ..      ..
    //    0,bottom      .. right,bottom
    let mut total = 0;
    for initial_y_velocity in target_area.bottom..=(-target_area.bottom - 1) {
        for initial_x_velocity in 0..=target_area.right {
            let mut x_vel = initial_x_velocity;
            let mut y_vel = initial_y_velocity;
            let mut x_pos = 0;
            let mut y_pos = 0;
            while y_pos >= target_area.bottom && x_pos <= target_area.right {
                x_pos += x_vel;
                y_pos += y_vel;
                if (target_area.left..=target_area.right).contains(&x_pos)
                    && (target_area.bottom..=target_area.top).contains(&y_pos)
                {
                    total += 1;
                    break;
                }
                if x_vel == 0 {
                    if x_pos < target_area.left {
                        break;
                    }
                } else {
                    x_vel -= 1;
                }
                y_vel -= 1;
            }
        }
    }
    total.to_string()
}
