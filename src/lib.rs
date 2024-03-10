use crate::sudoku::{checker, solver};
use crate::sudoku::{create_board, display_sudoku_board, read_sudoku_files};

mod sudoku;

pub fn run(sudoku_file_dir: String, sudoku_file_name: String) {
    let sudoku_line = read_sudoku_files(sudoku_file_dir, &sudoku_file_name).unwrap();
    let mut sudoku = create_board(sudoku_line).unwrap();
    let algorithm = String::from("brute_force");
    display_sudoku_board(&sudoku);
    solver::solve_sudoku(&mut sudoku, algorithm);
    checker::check_sudoku_completed(&sudoku);
    display_sudoku_board(&sudoku);
}
