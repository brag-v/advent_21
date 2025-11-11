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

#[test]
fn test_day5_task1() {
    assert_task!(day5::task1,
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    -> "5");
}

#[test]
fn test_day5_task2() {
    assert_task!(day5::task2,
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    -> "12");
}

#[test]
fn test_day6_task1() {
    assert_task!(day6::task1,
        "3,4,3,1,2",
    -> "5934");
}

#[test]
fn test_day6_task2() {
    assert_task!(day6::task2,
        "3,4,3,1,2",
    -> "26984457539");
}

#[test]
fn test_day7_task1() {
    assert_task!(day7::task1,
		"16,1,2,0,4,2,7,1,2,14",
    -> "37");
}

#[test]
fn test_day7_task2() {
    assert_task!(day7::task2,
		"16,1,2,0,4,2,7,1,2,14",
    -> "168");
}

#[test]
fn test_day8_task1() {
    assert_task!(day8::task1,
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    -> "26");
}

#[test]
fn test_day8_task2() {
    assert_task!(day8::task2,
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    -> "61229");
}

#[test]
fn test_day9_task1() {
    assert_task!(day9::task1,
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    -> "15");
}

#[test]
fn test_day9_task2() {
    assert_task!(day9::task2,
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    -> "1134");
}

#[test]
fn test_day10_task1() {
    assert_task!(day10::task1,
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    -> "26397");
}

#[test]
fn test_day10_task2() {
    assert_task!(day10::task2,
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    -> "288957");
}

#[test]
fn test_day11_task1() {
    assert_task!(day11::task1,
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    -> "1656");
}

#[test]
fn test_day11_task2() {
    assert_task!(day11::task2,
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    -> "195");
}

#[test]
fn test_day12_task1() {
    assert_task!(day12::task1,
        "start-A",
        "start-b",
        "A-c",
        "A-b",
        "b-d",
        "A-end",
        "b-end",
    -> "10");

    assert_task!(day12::task1,
        "dc-end",
        "HN-start",
        "start-kj",
        "dc-start",
        "dc-HN",
        "LN-dc",
        "HN-end",
        "kj-sa",
        "kj-HN",
        "kj-dc",
    -> "19");

    assert_task!(day12::task1,
        "fs-end",
        "he-DX",
        "fs-he",
        "start-DX",
        "pj-DX",
        "end-zg",
        "zg-sl",
        "zg-pj",
        "pj-he",
        "RW-he",
        "fs-DX",
        "pj-RW",
        "zg-RW",
        "start-pj",
        "he-WI",
        "zg-he",
        "pj-fs",
        "start-RW",
    -> "226");
}

#[test]
fn test_day12_task2() {
    assert_task!(day12::task2,
        "start-A",
        "start-b",
        "A-c",
        "A-b",
        "b-d",
        "A-end",
        "b-end",
    -> "36");

    assert_task!(day12::task2,
        "dc-end",
        "HN-start",
        "start-kj",
        "dc-start",
        "dc-HN",
        "LN-dc",
        "HN-end",
        "kj-sa",
        "kj-HN",
        "kj-dc",
    -> "103");

    assert_task!(day12::task2,
        "fs-end",
        "he-DX",
        "fs-he",
        "start-DX",
        "pj-DX",
        "end-zg",
        "zg-sl",
        "zg-pj",
        "pj-he",
        "RW-he",
        "fs-DX",
        "pj-RW",
        "zg-RW",
        "start-pj",
        "he-WI",
        "zg-he",
        "pj-fs",
        "start-RW",
    -> "3509");
}
