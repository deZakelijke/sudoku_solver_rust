use std::env;
use sudoku_solver_rust::run;

fn main() {
    let sudoku_file_dir: String = env::var("SUDOKU_FILE_DIR").unwrap_or("data/".to_string());
    let sudoku_file_name = "0.txt".to_string();
    run(sudoku_file_dir, sudoku_file_name);
}
