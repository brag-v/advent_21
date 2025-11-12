pub fn task1(input: String) -> String
{
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for line in input.lines() {
        let (direction, num) = line.split_once(' ').unwrap();
        let num: i32 = num.parse().unwrap();
        match direction.chars().next() {
            Some('f') => x += num,
            Some('u') => y -= num,
            Some('d') => y += num,
            _ => panic!("Invalid direction"),
        }
    }

    (x.abs() * y.abs()).to_string()
}

pub fn task2(input: String) -> String
{
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;
    for line in input.lines() {
        let (direction, num) = line.split_once(' ').unwrap();
        let num: i32 = num.parse().unwrap();
        match direction.chars().next() {
            Some('f') => {
                x += num;
                y += num * aim;
            }
            Some('u') => aim -= num,
            Some('d') => aim += num,
            _ => panic!("Invalid direction"),
        }
    }

    (x.abs() * y.abs()).to_string()
}
