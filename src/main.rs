use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::Instant,
};

#[cfg(test)]
mod test;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Unhapppy paths for wrong arguments or IO errors
    if args.len() != 2 {
        return Err("Please provide a task to solve".into());
    }

    let Some((day, task)) = args[1].split_once('.') else {
        return Err("Please provide a task in the format <day number>.<task number>".into());
    };

    let path = format!("./data/day{day}.txt");

    let input = read_lines(&path)?;

    // Select solver for the provided task
    let solver = match (day, task) {
        ("1", "1") => day1::task1,
        ("1", "2") => day1::task2,
        ("2", "1") => day2::task1,
        ("2", "2") => day2::task2,
        ("3", "1") => day3::task1,
        ("3", "2") => day3::task2,
        ("4", "1") => day4::task1,
        ("4", "2") => day4::task2,
        ("5", "1") => day5::task1,
        ("5", "2") => day5::task2,
        ("6", "1") => day6::task1,
        ("6", "2") => day6::task2,
        ("7", "1") => day7::task1,
        ("7", "2") => day7::task2,
        ("8", "1") => day8::task1,
        ("8", "2") => day8::task2,
        ("9", "1") => day9::task1,
        ("9", "2") => day9::task2,
        ("10", "1") => day10::task1,
        ("10", "2") => day10::task2,
        _ => return Err(format!("Day {day} task {task} is not implemented").into()),
    };

    // Solve task, and measure runtime
    let start_time = Instant::now();
    let result = solver(input);
    let runtime = start_time.elapsed();

    println!("Result: {result}");
    println!("Elapsed time: {:.3} ms", runtime.as_secs_f64() * 1000.);

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
