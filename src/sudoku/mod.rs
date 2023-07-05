use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod solver;
pub struct Sudoku {
    board: Vec<Vec<char>>,
}

pub fn read_sudoku_files(mut file_dir: String, file_name: &str) -> Result<String, ()> {
    file_dir.push_str(file_name);

    let sudoku_file = File::open(file_dir).unwrap();
    let buffer = BufReader::new(sudoku_file);
    let sudoku_line = buffer.lines().next().unwrap().unwrap();
    Ok(sudoku_line)
}

pub fn create_board(sudoku_line: String) -> Result<Sudoku, ()> {
    let mut board = Vec::new();
    for (index, digit) in sudoku_line.chars().enumerate() {
        if index % 9 == 0 {
            board.push(Vec::new());
        }
        board[index / 9].push(digit);
    }
    Ok(Sudoku { board })
}

pub fn display_sudoku_board(sudoku: &Sudoku) {
    let mut board_string = String::new();
    for (column_index, line) in sudoku.board.iter().enumerate() {
        for (row_index, number) in line.iter().enumerate() {
            board_string.push(number.clone());
            if row_index == 2 || row_index == 5 {
                board_string.push('|');
            }
            if row_index == 8 {
                board_string.push('\n');
            }
        }
        if column_index == 2 || column_index == 5 {
            board_string.push_str("---+---+---\n");
        }
    }
    println!("{board_string}");
}
