use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

pub mod checker;
pub mod solver;
pub struct Sudoku {
    board: Vec<Vec<char>>,
}

/// Read the first sudoku from a file that contains sudokus as a string of 81 chars on
/// one line.
pub fn read_single_sudoku_from_file(mut file_dir: String, file_name: &str) -> Result<String, ()> {
    file_dir.push_str(file_name);

    let sudoku_file = File::open(file_dir).unwrap();
    let buffer = BufReader::new(sudoku_file);
    let sudoku_line = buffer.lines().next().unwrap().unwrap();
    return Ok(sudoku_line);
}
pub fn read_all_sudokus_from_file(
    mut file_dir: String,
    file_name: &str,
) -> Result<Vec<String>, ()> {
    file_dir.push_str(file_name);
    let mut sudoku_lines: Vec<String> = Vec::new();

    for line in read_to_string(file_dir).unwrap().lines() {
        sudoku_lines.push(line.to_string());
    }
    return Ok(sudoku_lines);
}

impl Sudoku {
    pub fn create_board(sudoku_line: String) -> Result<Self, ()> {
        let mut board = Vec::new();
        for (index, digit) in sudoku_line.chars().enumerate() {
            if index % 9 == 0 {
                board.push(Vec::new());
            }
            board[index / 9].push(digit);
        }
        Ok(Sudoku { board })
    }

    pub fn display_sudoku_board(&self) {
        let mut board_string = String::new();
        for (column_index, line) in self.board.iter().enumerate() {
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
}
