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
