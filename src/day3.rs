use std::fmt::Debug;

/// Finds the total number of 1s by position in the input, as well as the length of the input
fn count_numbers<I>(input: I) -> (Vec<u32>, u32)
where
    I: Iterator<Item = String>,
{
    let mut peekable_input = input.peekable();
    let measure_length = peekable_input.peek().as_ref().unwrap().len();
    let mut counts = vec![0; measure_length];
    let mut length = 0;
    for line in peekable_input {
        for (i, bit) in line.chars().enumerate() {
            if bit == '1' {
                counts[i] += 1;
            }
        }
        length += 1;
    }
    (counts, length)
}

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let (counts, length) = count_numbers(input.map(|line| line.unwrap()));
    let cutoff = length / 2;
    let number_lengths = counts.len();
    let gamma_rate: u64 = counts
        .into_iter()
        .map(|count| u64::from(count > cutoff))
        .fold(0, |acc, bit| acc * 2 + bit);
    let epsilon_rate = ((!gamma_rate) << (64 - number_lengths)) >> (64 - number_lengths);
    let power_consuption = gamma_rate * epsilon_rate;
    power_consuption.to_string()
}

/// Split list into two using the number at index.
/// The lists with a 1 at index is the first element returned,
/// the list with a 0 at index is the second
fn split_list(measurements: Vec<&[u8]>, index: usize) -> (Vec<&[u8]>, Vec<&[u8]>) {
    let mut ones = Vec::new();
    let mut zeros = Vec::new();
    for measurement in measurements {
        if measurement[index] == b'1' {
            ones.push(measurement);
        } else {
            zeros.push(measurement);
        }
    }
    (zeros, ones)
}

/// Finds the rating of the measurements by repeatedly applying the keep majority/minority rule to
/// each index until there is only one measurement left.
fn find_rating(mut measurements: Vec<&[u8]>, keep_majority: bool) -> u64 {
    for index in 1..measurements[0].len() {
        let (zeros, ones) = split_list(measurements, index);
        if (zeros.len() > ones.len()) ^ !keep_majority {
            measurements = zeros;
        } else {
            measurements = ones;
        }
        if measurements.len() == 1 {
            break;
        }
    }
    measurements[0]
        .iter()
        .fold(0, |acc, bit| acc * 2 + u64::from(*bit == b'1'))
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let binding: Vec<String> = input.map(|line| line.unwrap()).collect();
    let measurements: Vec<&[u8]> = binding.iter().map(|line| line.as_bytes()).collect();
    let (zeros, ones) = split_list(measurements, 0);
    let (majority, minority) = if zeros.len() > ones.len() {
        (zeros, ones)
    } else {
        (ones, zeros)
    };

    let oxygen_generator_rating: u64 = find_rating(majority, true);
    let co2_scrubber_rating: u64 = find_rating(minority, false);

    (oxygen_generator_rating * co2_scrubber_rating).to_string()
}
