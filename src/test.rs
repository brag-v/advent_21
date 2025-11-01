use super::*;

macro_rules! evaluate_task {
    ($task:path,$($line:literal,)*) => {{
        let lines: Vec<Result<String, ()>> = vec![$(Ok(String::from($line)), )*];
        $task(lines.into_iter())
    }};
    ($task:path,$($line:literal,)* with $($args:literal,)+) => {{
        let lines: Vec<Result<String, ()>> = vec![$(Ok(String::from($line)), )*];
        $task(lines.into_iter(), $($args,)+)
    }}
}

macro_rules! assert_task {
    ($task:path,$($line:literal,)* -> $expected:literal) => {{
        let solution = evaluate_task!($task, $($line,)+);
        assert_eq!(solution, $expected.to_string());
    }};
    ($task:path,$($line:literal,)* with $($args:literal,)+ -> $expected:literal) => {{
        let solution = evaluate_task!($task, $($line,)+ with $($args,)+);
        assert_eq!(solution, $expected.to_string());
    }}
}

#[test]
fn test_day1_task1() {
    assert_task!(day1::task1,
        "199",
        "200",
        "208",
        "210",
        "200",
        "207",
        "240",
        "269",
        "260",
        "263",
    -> "7");
}

#[test]
fn test_day1_task2() {
    assert_task!(day1::task2,
        "199",
        "200",
        "208",
        "210",
        "200",
        "207",
        "240",
        "269",
        "260",
        "263",
    -> "5");
}

#[test]
fn test_day2_task1() {
    assert_task!(day2::task1,
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    -> "150");
}

#[test]
fn test_day2_task2() {
    assert_task!(day2::task2,
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    -> "900");
}

#[test]
fn test_day3_task1() {
    assert_task!(day3::task1,
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    -> "198");
}

#[test]
fn test_day3_task2() {
    assert_task!(day3::task2,
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    -> "230");
}

#[test]
fn test_day4_task1() {
    assert_task!(day4::task1,
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
        "",
        " 3 15  0  2 22",
        " 9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
    -> "4512");
}

#[test]
fn test_day4_task2() {
    assert_task!(day4::task2,
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
        "",
        " 3 15  0  2 22",
        " 9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
    -> "1924");
}
