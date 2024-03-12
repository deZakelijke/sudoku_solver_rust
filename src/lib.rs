use crate::sudoku::{checker, solver};
use crate::sudoku::{
    create_board, display_sudoku_board, read_all_sudokus_from_file, read_single_sudoku_from_file,
};

mod sudoku;

pub fn run(sudoku_file_dir: String, sudoku_file_name: String) {
    let all = true;
    if all {
        let sudoku_lines = read_all_sudokus_from_file(sudoku_file_dir, &sudoku_file_name).unwrap();
        for (sudoku_number, sudoku_line) in sudoku_lines.iter().enumerate() {
            println!("Solving sudoku {sudoku_number}");
            let mut sudoku = create_board(sudoku_line.clone()).unwrap();
            let algorithm = String::from("simple");
            solver::solve_sudoku(&mut sudoku, algorithm);
            checker::check_sudoku_completed(&sudoku);
        }
    } else {
        let sudoku_line = read_single_sudoku_from_file(sudoku_file_dir, &sudoku_file_name).unwrap();
        let mut sudoku = create_board(sudoku_line).unwrap();
        let algorithm = String::from("simple");
        display_sudoku_board(&sudoku);
        solver::solve_sudoku(&mut sudoku, algorithm);
        checker::check_sudoku_completed(&sudoku);
        display_sudoku_board(&sudoku);
    }
}
