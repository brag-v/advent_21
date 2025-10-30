use std::fmt::Debug;

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let mut last = None;
    let mut count = 0;
    for line in input {
        let num: u16 = line.unwrap().parse().unwrap();
        if let Some(last_num) = last
            && num > last_num
        {
            count += 1;
        }
        last = Some(num);
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
        let last = &mut window[i % 3];
        let num = line.unwrap().parse::<u32>().unwrap();
        if let Some(last) = last
            && num > *last
        {
            count += 1;
        }
        *last = Some(num);
    }
    count.to_string()
}
