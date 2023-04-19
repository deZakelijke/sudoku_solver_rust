use sudoku_solver_rust::{create_board, display_sudoku_board, read_sudoku_files};
fn main() {
    let sudoku_line = read_sudoku_files().unwrap();
    let sudoku_board = create_board(sudoku_line).unwrap();
    display_sudoku_board(sudoku_board);
}
