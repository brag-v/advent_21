fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let (signal, output) = line.split_once(" | ").unwrap();
            (
                signal.split(' ').map(|s| s.to_string()).collect(),
                output.split(' ').map(|s| s.to_string()).collect(),
            )
        })
        .collect()
}

fn count_identifiable_digits(signal: &[String]) -> usize {
    signal
        .iter()
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count()
}

pub fn task1(input: String) -> String {
    let entries = parse_input(&input);
    entries
        .iter()
        .map(|(_signal, output)| count_identifiable_digits(output))
        .sum::<usize>()
        .to_string()
}

/// In set notation computes |lhs - rhs|
fn difference(lhs: &str, rhs: &str) -> usize {
    lhs.chars()
        .filter(|lhc| !rhs.chars().any(|rhc| rhc == *lhc))
        .count()
}

fn decode_digit(
    num: &str,
    one: Option<&str>,
    seven: Option<&str>,
    four: Option<&str>,
) -> Option<usize> {
    // xx: 1
    // xxx: 7
    // xxxx: 4
    // xxxxx: 2, 5, 3
    // xxxxxx: 6, 9
    // xxxxxxx: 8
    match num.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        5 => match (one, seven, four) {
            // use a pair of a digit contained in three and 4 (the digit that distingiushes 2 and 5) to
            // distinguish all the 5-segment number
            (_, _, None) => None,              // doesn't distinguish 2 and 5
            (None, None, Some(_four)) => None, // doesn't distinguish 3 and 5
            (Some(contained_in_three), _, Some(four))
            | (None, Some(contained_in_three), Some(four)) => {
                if difference(contained_in_three, num) == 0 {
                    Some(3)
                } else if difference(four, num) == 1 {
                    Some(5)
                } else {
                    Some(2)
                }
            }
        },
        6 => match (one, seven, four) {
            (Some(helper), _, _) | (None, Some(helper), _) | (None, None, Some(helper)) => {
                // if 6-segment digit completely contains any of 1, 7 or 4 it's a 9,
                // otherwise it's a 6
                if difference(helper, num) == 0 {
                    Some(9)
                } else {
                    Some(6)
                }
            }
            (None, None, None) => None,
        },
        7 => Some(8),
        _ => None,
    }
}

fn decode_output(
    output: &[String],
    one: Option<&str>,
    seven: Option<&str>,
    four: Option<&str>,
) -> usize {
    let mut output_num = 0;
    for num in output {
        match decode_digit(num, one, seven, four) {
            Some(num) => output_num = 10 * output_num + num,
            None => {
                panic!(
                    "{}",
                    format!("Unable to parse num {num} with {one:?}, {seven:?}, {four:?}")
                )
            }
        }
    }
    output_num
}

fn gather_identifiable_digits<'a>(
    signal: &'a [String],
    output: &'a [String],
) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>) {
    let mut one = None;
    let mut seven = None;
    let mut four = None;

    signal.iter().for_each(|num| match num.len() {
        2 => one = Some(num.as_str()),
        3 => seven = Some(num.as_str()),
        4 => four = Some(num.as_str()),
        _ => (),
    });

    output.iter().for_each(|num| match num.len() {
        2 => one = Some(num.as_str()),
        3 => seven = Some(num.as_str()),
        4 => four = Some(num.as_str()),
        _ => (),
    });

    (one, seven, four)
}

pub fn task2(input: String) -> String {
    let entries = parse_input(&input);
    let mut sum = 0;
    for (signal, output) in entries {
        let (one, seven, four) = gather_identifiable_digits(&signal, &output);
        match (one, seven, four) {
            (Some(_), Some(_), Some(_)) => (),
            _ => println!("{one:?}, {seven:?}, {four:?}"),
        }
        sum += decode_output(&output, one, seven, four);
    }
    sum.to_string()
}
