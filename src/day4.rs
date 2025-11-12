use std::fmt::Debug;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Debug)]
struct Board {
    numbers: [[u8; WIDTH]; HEIGHT],
    marks: [[bool; WIDTH]; HEIGHT],
}

fn get_boards_and_sequence(input: String) -> (Vec<Board>, Vec<u8>) {
    let (sequence_input, boards_input) = input.split_once("\n\n").unwrap();
    let sequence: Vec<u8> = sequence_input
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let boards = boards_input
        .split("\n\n")
        .map(|board| {
            let mut numbers = [[0; WIDTH]; HEIGHT];
            board.lines().enumerate().for_each(|(y, line)| {
                line.split_whitespace()
                    .inspect(|num| println!("{num}"))
                    .map(|num| num.parse().unwrap())
                    .enumerate()
                    .for_each(|(x, num)| numbers[y][x] = num);
            });
            Board {
                numbers,
                marks: [[false; WIDTH]; HEIGHT],
            }
        })
        .collect();

    (boards, sequence)
}

/// Checks if a board is winning by looking at squares in the row/column of (x, y).
/// Should be used after marking the square at (x, y).
fn check_winning(x: usize, y: usize, board: &Board) -> bool {
    (0..WIDTH).all(|row_x| board.marks[y][row_x]) || (0..HEIGHT).all(|col_y| board.marks[col_y][x])
}

/// Mark number if present on the board, returns true if marking the number
/// resulted in a winning board.
fn update_and_check_board(number: u8, board: &mut Board) -> bool {
    for (y, row) in board.numbers.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if *num == number {
                board.marks[y][x] = true;
                if check_winning(x, y, board) {
                    return true;
                }
                return false;
            }
        }
    }
    false
}

/// Gives the score of a (winning) board, which is calculated by summing all unmarked squares and
/// multiplying the sum by the last number called.
fn board_score(board: &Board, number: u8) -> u64 {
    board
        .numbers
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _num)| !board.marks[y][*x])
                .map(|(_x, num)| u64::from(*num))
                .sum::<u64>()
        })
        .sum::<u64>()
        * u64::from(number)
}

pub fn task1(input: String) -> String {
    let (mut boards, sequence) = get_boards_and_sequence(input);
    for num in sequence {
        for board in &mut boards {
            if update_and_check_board(num, board) {
                return board_score(board, num).to_string();
            }
        }
    }
    "No winners".to_string()
}

pub fn task2(input: String) -> String {
    let (mut boards, sequence) = get_boards_and_sequence(input);
    let mut last_board = None;
    let mut last_num_i = 0;
    for board in &mut boards {
        for (i, num) in sequence.iter().enumerate() {
            if update_and_check_board(*num, board) {
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
        Some(last_winner) => board_score(last_winner, sequence[last_num_i]).to_string(),
    }
}
