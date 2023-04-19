use std::fs::File;
use std::io::{BufRead, BufReader};

struct Sudoku {}

fn main() {
    let sudoku_line = read_sudoku_files().unwrap();
    let sudoku_board = create_board(sudoku_line);
    // Display a board
}

fn read_sudoku_files() -> Result<String, ()> {
    let mut data_path: String = "data/".to_owned();
    let file_name: &str = "0.txt";
    data_path.push_str(file_name);

    let sudoku_file = File::open(data_path).unwrap();
    let buffer = BufReader::new(sudoku_file);
    // let line = buffer.next();
    let sudoku_line = buffer.lines().next().unwrap().unwrap();
    Ok(sudoku_line)
}

fn create_board(sudoku_line: String) -> Result<Sudoku, ()> {
    Ok(Sudoku {})
}
