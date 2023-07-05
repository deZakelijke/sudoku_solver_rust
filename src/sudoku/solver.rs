use crate::sudoku::{create_board, Sudoku};
pub fn solve_sudoku(mut _sudoku: &Sudoku) {}

fn check_block_correctness(sudoku: &Sudoku, index: usize) -> bool {
    let mut block = Vec::new();
    for i in 0..9 {
        block.push(sudoku.board[i % 3 + index / 3][i / 3 + index % 3]);
    }
    let dups = check_duplicates_in_slice(&block);
    let zeros = block.contains(&'0');
    !dups && !zeros
}
fn check_row_correctness(sudoku: &Sudoku, index: usize) -> bool {
    let row = &sudoku.board[index];
    let dups = check_duplicates_in_slice(row);
    let zeros = row.contains(&'0');
    !dups && !zeros
}
fn check_column_correctness(sudoku: &Sudoku, index: usize) -> bool {
    let mut column = Vec::new();
    for i in 0..sudoku.board.len() {
        column.push(sudoku.board[i][index]);
    }
    let dups = check_duplicates_in_slice(&column);
    let zeros = column.contains(&'0');
    !dups && !zeros
}
fn check_duplicates_in_slice(slice: &Vec<char>) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}

pub fn check_correctness_of_sudoku(sudoku: &Sudoku) -> bool {
    for index in 0..9 {
        if !check_row_correctness(sudoku, index) {
            return false;
        }
        if !check_block_correctness(sudoku, index) {
            return false;
        }
        if !check_column_correctness(sudoku, index) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirm_find_duplicates() {
        let vec_with_duplicates = vec!['a', 'b', 'b'];
        assert!(check_duplicates_in_slice(&vec_with_duplicates));
    }

    #[test]
    fn confirm_no_duplicates() {
        let vec_without_duplicates = vec!['a', 'b', 'c'];
        assert_eq!(check_duplicates_in_slice(&vec_without_duplicates), false);
    }

    #[test]
    fn row_not_correct() {
        let sudoku = create_board(
            "123456788000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        )
        .unwrap();
        assert_eq!(check_row_correctness(&sudoku, 0), false);
    }

    #[test]
    fn row_correct() {
        let sudoku = create_board(
            "123456789000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        )
        .unwrap();
        assert!(check_row_correctness(&sudoku, 0));
    }

    #[test]
    fn column_not_correct() {
        let sudoku = create_board(
            "100000000200000000300000000400000000500000000600000000700000000800000000800000000"
                .to_string(),
        )
        .unwrap();
        assert_eq!(check_column_correctness(&sudoku, 0), false);
    }

    #[test]
    fn column_correct() {
        let sudoku = create_board(
            "100000000200000000300000000400000000500000000600000000700000000800000000900000000"
                .to_string(),
        )
        .unwrap();
        assert!(check_column_correctness(&sudoku, 0));
    }

    #[test]
    fn block_not_correct() {
        let sudoku = create_board(
            "123000000456000000788000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        )
        .unwrap();
        assert_eq!(check_block_correctness(&sudoku, 0), false);
    }

    #[test]
    fn block_correct() {
        let sudoku = create_board(
            "123000000456000000789000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        )
        .unwrap();
        assert!(check_block_correctness(&sudoku, 0));
    }
}