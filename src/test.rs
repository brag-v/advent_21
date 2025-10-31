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
