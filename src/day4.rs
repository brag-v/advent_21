use std::fmt::Debug;

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;

#[derive(Debug)]
struct Board {
    numbers: [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
    marks: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
}

fn get_sequence_and_board<I, E>(mut input: I) -> (Vec<u8>, Vec<Board>)
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let sequence: Vec<u8> = input
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    let mut boards = Vec::new();

    while let Some(_new_line) = input.next() {
        let mut board = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
        input
            .by_ref()
            .take(BOARD_HEIGHT)
            .enumerate()
            .for_each(|(y, line)| {
                line.unwrap()
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .enumerate()
                    .for_each(|(x, num)| board[y][x] = num);
            });
        boards.push(Board {
            numbers: board,
            marks: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
        });
    }

    (sequence, boards)
}

fn check_marks(x: usize, y: usize, board: &Board) -> bool {
    (0..BOARD_WIDTH).all(|row_x| board.marks[y][row_x])
        || (0..BOARD_HEIGHT).all(|col_y| board.marks[col_y][x])
}

fn update_board_and_check_board(number: u8, board: &mut Board) -> bool {
    for (y, row) in board.numbers.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if *num == number {
                board.marks[y][x] = true;
                if check_marks(x, y, board) {
                    return true;
                }
                return false;
            }
        }
    }
    false
}

fn calculate_board_score(board: &Board, number: u8) -> u64 {
    board
        .numbers
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _num)| !board.marks[y][*x])
                .map(|(_x, num)| *num as u64)
                .sum::<u64>()
        })
        .sum::<u64>()
        * (number as u64)
}

pub fn task1<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let (sequence, mut boards) = get_sequence_and_board(input);
    for num in sequence {
        for board in &mut boards {
            if update_board_and_check_board(num, board) {
                return calculate_board_score(board, num).to_string();
            }
        }
    }
    "No winners".to_string()
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    let (sequence, mut boards) = get_sequence_and_board(input);
    let mut last_board = None;
    let mut last_num_i = 0;
    for board in &mut boards {
        for (i, num) in sequence.iter().enumerate() {
            if update_board_and_check_board(*num, board) {
                if i > last_num_i {
                    last_board = Some(board);
                    last_num_i = i;
                }
                break;
            }
        }
    }

    match last_board {
        None => "No winners".to_string(),
        Some(last_winner) => calculate_board_score(last_winner, sequence[last_num_i]).to_string(),
    }
}
