use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pair {
    left: u8,
    right: u8,
}

fn get_rules_pairs_and_endpoints(
    input: &str,
) -> (HashMap<Pair, (Pair, Pair)>, HashMap<Pair, u64>, Pair) {
    let (initial_input, rules_input) = input.split_once("\n\n").unwrap();
    let mut pairs = HashMap::new();
    let bytes = initial_input.bytes().collect::<Box<[u8]>>();
    bytes
        .windows(2)
        .map(|pair| Pair {
            left: pair[0],
            right: pair[1],
        })
        .for_each(|pair| *pairs.entry(pair).or_insert(0) += 1);

    let endpoint = Pair {
        left: bytes[0],
        right: bytes[bytes.len() - 1],
    };

    let rules = rules_input
        .lines()
        .map(|line| line.as_bytes())
        .map(|bytes| {
            let in_pair = Pair {
                left: bytes[0],
                right: bytes[1],
            };
            let out_pair_1 = Pair {
                left: bytes[0],
                right: bytes[6],
            };
            let out_pair_2 = Pair {
                left: bytes[6],
                right: bytes[1],
            };
            (in_pair, (out_pair_1, out_pair_2))
        })
        .collect::<HashMap<Pair, (Pair, Pair)>>();

    (rules, pairs, endpoint)
}

fn perform_pair_insertion(
    current_counts: &mut HashMap<Pair, u64>,
    next_counts: &mut HashMap<Pair, u64>,
    rules: &HashMap<Pair, (Pair, Pair)>,
) {
    for (pair, count) in current_counts.iter_mut() {
        let (pair1, pair2) = rules.get(pair).unwrap();
        *next_counts.entry(*pair1).or_insert(0) += *count;
        *next_counts.entry(*pair2).or_insert(0) += *count;
        *count = 0;
    }
}

fn get_letter_counts(pairs: &HashMap<Pair, u64>, endpints: Pair) -> HashMap<u8, u64> {
    let mut letter_counts: HashMap<u8, u64> = HashMap::new();
    for (pair, count) in pairs.iter() {
        *letter_counts.entry(pair.left).or_insert(0) += *count;
        *letter_counts.entry(pair.right).or_insert(0) += *count;
    }
    // Everything except endpoints are double counted
    *letter_counts.get_mut(&endpints.left).unwrap() += 1;
    *letter_counts.get_mut(&endpints.right).unwrap() += 1;
    for count in letter_counts.values_mut() {
        *count /= 2
    }
    letter_counts
}

fn perform_tasks(input: String, steps: usize) -> String {
    // TODO: using all possible pairs as entries, pair count can be encoded in a vector and rules
    // in a matrix, using tricks like repeaded squaring of the rule matrix.
    // However, the running time is quite low already, even in task 2
    let (rules, mut initial_pairs, endpoints) = get_rules_pairs_and_endpoints(&input);
    let mut current = &mut initial_pairs;
    let mut next = &mut HashMap::new();
    for _ in 0..steps {
        perform_pair_insertion(current, next, &rules);
        (current, next) = (next, current);
    }
    let letter_counts = get_letter_counts(current, endpoints);
    let min_freq = letter_counts.values().min().unwrap();
    let max_freq = letter_counts.values().max().unwrap();
    (max_freq - min_freq).to_string()
}

pub fn task1(input: String) -> String {
    perform_tasks(input, 10)
}

pub fn task2(input: String) -> String {
    perform_tasks(input, 40)
}
