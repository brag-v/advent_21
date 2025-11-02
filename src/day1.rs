use std::fmt::Debug;

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let mut prev = None;
    let mut count = 0;
    for line in input {
        let next: u16 = line.unwrap().parse().unwrap();
        if let Some(prev) = prev
            && next > prev
        {
            count += 1;
        }
        prev = Some(next);
    }
    count.to_string()
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let mut window = [None, None, None];
    let mut count = 0;
    for (i, line) in input.enumerate() {
        // weather or not the sum increases or not is dependent only on the relationship
        // between the newly added and newly removed number
        let removed = &mut window[i % 3];
        let added = line.unwrap().parse::<u32>().unwrap();
        if let Some(removed) = removed
            && added > *removed
        {
            count += 1;
        }
        *removed = Some(added);
    }
    count.to_string()
}
