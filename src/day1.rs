pub fn task1(input: String) -> String {
    let mut prev = None;
    let mut count = 0;
    for line in input.lines() {
        let next: u16 = line.parse().unwrap();
        if let Some(prev) = prev
            && next > prev
        {
            count += 1;
        }
        prev = Some(next);
    }
    count.to_string()
}

pub fn task2(input: String) -> String {
    let mut window = [None, None, None];
    let mut count = 0;
    for (i, line) in input.lines().enumerate() {
        // weather or not the sum increases or not is dependent only on the relationship
        // between the newly added and newly removed number
        let removed = &mut window[i % 3];
        let added = line.parse::<u32>().unwrap();
        if let Some(removed) = removed
            && added > *removed
        {
            count += 1;
        }
        *removed = Some(added);
    }
    count.to_string()
}
