use crate::sudoku::checker;
use crate::sudoku::Sudoku;
use std::error::Error;

pub fn solve_sudoku(sudoku: &mut Sudoku, algorithm: String) {
    if algorithm == "simple" {
        simple_solver(sudoku);
    }
}

fn simple_solver(sudoku: &mut Sudoku) {
    fill_value_and_check(sudoku).unwrap();
}

fn fill_value_and_check(sudoku: &mut Sudoku) -> Result<&Sudoku, Box<dyn Error>> {
    if let Ok((row_index, column_index, options)) = choose_first_possible_value(sudoku) {
        for option in options.iter() {
            sudoku.board[row_index][column_index] = *option;
            if checker::check_correctness_of_sudoku(sudoku) {
                match fill_value_and_check(sudoku) {
                    Ok(_) => break,
                    Err(_) => {
                        sudoku.board[row_index][column_index] = '0';
                    }
                };
            }
        }
        Ok(sudoku)
    } else {
        Ok(sudoku)
    }
}

fn choose_first_possible_value(sudoku: &Sudoku) -> Result<(usize, usize, Vec<char>), ()> {
    for i in 0..sudoku.board.len() {
        for j in 0..sudoku.board[i].len() {
            if sudoku.board[i][j] == '0' {
                let possible_values = check_possible_values(sudoku, i, j);
                return Ok((i, j, possible_values));
            }
        }
    }
    Err(())
}

fn check_possible_values(sudoku: &Sudoku, row: usize, column: usize) -> Vec<char> {
    let existing_row_values = sudoku.board[row].iter().filter(|s| **s != '0');
    // let existing_column_values
    vec!['0']
}
