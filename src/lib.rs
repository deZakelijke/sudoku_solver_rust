use crate::sudoku::checker;
use crate::sudoku::{
    Sudoku, read_all_sudokus_from_file, read_single_sudoku_from_file,
};

mod sudoku;

pub fn run(sudoku_file_dir: String, sudoku_file_name: String) {
    let all = false;
    if all {
        let sudoku_lines = read_all_sudokus_from_file(sudoku_file_dir, &sudoku_file_name).unwrap();
        for (sudoku_number, sudoku_line) in sudoku_lines.iter().enumerate() {
            println!("Solving sudoku {sudoku_number}");
            let mut sudoku = Sudoku::create_board(sudoku_line.clone()).unwrap();
            let algorithm = String::from("simple");
            sudoku.solve_sudoku(algorithm);
            checker::check_sudoku_completed(&sudoku);
        }
    } else {
        let sudoku_line = read_single_sudoku_from_file(sudoku_file_dir, &sudoku_file_name).unwrap();
        let mut sudoku = Sudoku::create_board(sudoku_line).unwrap();
        let algorithm = String::from("simple");
        sudoku.display_sudoku_board();
        sudoku.solve_sudoku(algorithm);
        checker::check_sudoku_completed(&sudoku);
        sudoku.display_sudoku_board();
    }
}
