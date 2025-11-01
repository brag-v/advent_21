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

fn update_and_check_board(number: u8, boards: &mut Vec<Board>) -> Option<&Board> {
    for board in boards {
        'board: for (y, row) in board.numbers.iter().enumerate() {
            for (x, num) in row.iter().enumerate() {
                if *num == number {
                    board.marks[y][x] = true;
                    if check_marks(x, y, board) {
                        return Some(board);
                    }
                    break 'board;
                }
            }
        }
    }
    None
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
        if let Some(winner) = update_and_check_board(num, &mut boards) {
            return calculate_board_score(winner, num).to_string();
        }
    }
    "No winners".to_string()
}

pub fn task2<I, E>(input: I) -> String
where
    I: Iterator<Item = Result<String, E>>,
    E: Debug,
{
    todo!("Task not implemented");
}
