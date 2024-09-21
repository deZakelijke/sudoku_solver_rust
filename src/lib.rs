use crate::sudoku::{
    Sudoku, read_all_sudokus_from_file, read_single_sudoku_from_file,
};
use crate::sudoku::solver::SolverAlgorithm;

mod sudoku;

pub fn run(sudoku_file_dir: String, sudoku_file_name: String) {
    let all = true;
    let algorithm = SolverAlgorithm::EmptyCellHashMap;

    if all {
        let sudoku_lines = read_all_sudokus_from_file(sudoku_file_dir, &sudoku_file_name).unwrap();
        for (sudoku_number, sudoku_line) in sudoku_lines.iter().enumerate() {
            let mut sudoku = Sudoku::create_board(sudoku_line.clone()).unwrap();
            sudoku.solve_sudoku(&algorithm);
            if !sudoku.check_sudoku_completed() {
                println!("Solving sudoku {sudoku_number}");
                println!("Sudoku not solved correctly")
            }

        }
    } else {
        let sudoku_line = read_single_sudoku_from_file(sudoku_file_dir, &sudoku_file_name).unwrap();
        let mut sudoku = Sudoku::create_board(sudoku_line).unwrap();
        sudoku.display_sudoku_board();
        sudoku.solve_sudoku(&algorithm);
        if sudoku.check_sudoku_completed() {
            println!("Sudoku solved correctly!");
        } else {
            println!("Did not solve sudoku correctly");
        }
        sudoku.display_sudoku_board();
    }
}
