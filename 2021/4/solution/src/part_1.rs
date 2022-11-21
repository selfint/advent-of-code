use std::collections::HashMap;

type Number = i32;
type Board = Vec<Vec<Number>>;
type Position = (usize, usize);
type BoardID = usize;
type BoardMap = HashMap<Number, Vec<(BoardID, Position)>>;

pub fn solve<T, I>(mut input: I) -> Number
where
    T: Into<String>,
    I: Iterator<Item = T>,
{
    let numbers: String = input.next().expect("failed to get numbers").into();
    let numbers = numbers
        .split(',')
        .map(|n| n.parse::<Number>().expect("failed to parse number"));
    input.next().unwrap();

    // get boards
    let (board_map, mut boards) = get_boards(input);

    // run the numbers
    for number in numbers {
        let Some(number_boards) = board_map.get(&number) else {continue;};

        for (board_idx, position) in number_boards {
            let board = &mut boards[*board_idx];
            update_board(board, *position);

            if check_board(board) {
                return sum_board(board) * number;
            }
        }
    }

    unreachable!("no board won");
}

fn sum_board(board: &Board) -> Number {
    let mut sum = 0;
    for row in board {
        for number in row {
            if number >= &0 {
                sum += number;
            }
        }
    }

    sum
}

fn update_board(board: &mut Board, position: Position) {
    let (row, col) = position;
    board[row][col] = -1;
}

fn check_board(board: &Board) -> bool {
    let winner = -(board.len() as i32);

    // check rows
    if board.iter().any(|r| r.iter().sum::<i32>() == winner) {
        return true;
    }

    // check cols
    if board
        .iter()
        .fold(vec![0; board.len()], |mut cols, row| {
            for (col_idx, col_data) in row.iter().enumerate() {
                cols[col_idx] += col_data;
            }

            cols
        })
        .iter()
        .any(|&col_sum| col_sum == winner)
    {
        return true;
    }

    false
}

fn get_boards<T, I>(input: I) -> (BoardMap, Vec<Board>)
where
    T: Into<String>,
    I: Iterator<Item = T>,
{
    let mut boards = vec![];
    let mut board = vec![];
    let mut board_idx = 0;
    let mut board_map: BoardMap = HashMap::new();

    for line in input {
        let line: String = line.into();

        if line.is_empty() {
            boards.push(board.clone());
            board.clear();
            board_idx += 1;
            continue;
        }

        let line = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().expect("failed to parse number"))
            .collect::<Vec<_>>();

        let row = board.len();

        for (col, n) in line.iter().enumerate() {
            board_map
                .entry(*n)
                .or_default()
                .push((board_idx, (row, col)))
        }

        board.push(line);
    }

    (board_map, boards)
}

#[cfg(test)]
mod tests {
    use super::{get_boards, solve};

    const EXAMPLE_INPUT: [&str; 20] = [
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
        "",
        " 3 15  0  2 22",
        " 9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
        "",
    ];

    #[test]
    fn test_get_boards() {
        let (_, boards) = get_boards(EXAMPLE_INPUT.into_iter().skip(2));
        assert_eq!(boards.len(), 3);
    }

    #[test]
    fn test_example() {
        let lines = EXAMPLE_INPUT.into_iter();

        let solution = solve(lines);

        assert_eq!(solution, 4512);
    }
}
