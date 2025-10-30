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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Unhapppy paths for wrong arguments or IO errors
    if args.len() != 2 {
        return Err("Please provide a task to solve".into());
    }

    let (day, task) = match args[1].split_once(".") {
        Some(parsed) => parsed,
        None => {
            return Err("Please provide a task in the format <day number>.<task number>".into());
        }
    };

    let path = format!("./data/day{}.txt", day);

    let input = read_lines(&path)?;

    // Select solver for the provided task
    let solver = match (day, task) {
        ("1", "1") => day1::task1,
        ("1", "2") => day1::task2,
        _ => return Err(format!("Day {day} task {task} is not implemented").into()),
    };

    // Solve task, and measure runtime
    let start_time = Instant::now();
    let result = solver(input);
    let runtime = start_time.elapsed();

    println!("{result}");
    println!("Elapsed time: {:.3} ms", runtime.as_micros() as f64 * 1e-3);

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
