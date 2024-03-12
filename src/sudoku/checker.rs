use crate::sudoku::Sudoku;
fn check_block_correctness(sudoku: &Sudoku, index: usize) -> bool {
    let mut block = Vec::new();
    for i in 0..9 {
        block.push(sudoku.board[i % 3 + (index / 3) * 3][i / 3 + (index % 3) * 3]);
    }
    let dups = check_duplicates_in_slice(&block);
    !dups
}
fn check_row_correctness(sudoku: &Sudoku, index: usize) -> bool {
    let row = &sudoku.board[index];
    let dups = check_duplicates_in_slice(row);
    !dups
}
fn check_column_correctness(sudoku: &Sudoku, index: usize) -> bool {
    let mut column = Vec::new();
    for i in 0..sudoku.board.len() {
        column.push(sudoku.board[i][index]);
    }
    let dups = check_duplicates_in_slice(&column);
    !dups
}
fn check_duplicates_in_slice(slice: &Vec<char>) -> bool {
    let slice: Vec<char> = slice.iter().filter(|s| **s != '0').map(|s| *s).collect();
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

pub fn check_sudoku_is_filled(sudoku: &Sudoku) -> bool {
    for row in sudoku.board.iter() {
        if row.contains(&'0') {
            return false;
        }
    }
    true
}

pub fn check_sudoku_completed(sudoku: &Sudoku) -> bool {
    check_sudoku_is_filled(sudoku) && check_correctness_of_sudoku(sudoku)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sudoku::create_board;

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
            "123456788\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000"
                .to_string(),
        )
        .unwrap();
        assert_eq!(check_row_correctness(&sudoku, 0), false);
    }

    #[test]
    fn row_correct() {
        let sudoku = create_board(
            "123456789\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000"
                .to_string(),
        )
        .unwrap();
        assert!(check_row_correctness(&sudoku, 0));
    }

    #[test]
    fn column_not_correct() {
        let sudoku = create_board(
            "100000000\
             200000000\
             300000000\
             400000000\
             500000000\
             600000000\
             700000000\
             800000000\
             800000000"
                .to_string(),
        )
        .unwrap();
        assert_eq!(check_column_correctness(&sudoku, 0), false);
    }

    #[test]
    fn column_correct() {
        let sudoku = create_board(
            "100000000\
             200000000\
             300000000\
             400000000\
             500000000\
             600000000\
             700000000\
             800000000\
             900000000"
                .to_string(),
        )
        .unwrap();
        assert!(check_column_correctness(&sudoku, 0));
    }

    #[test]
    fn block_not_correct() {
        let sudoku = create_board(
            "123000000\
             456000000\
             788000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000"
                .to_string(),
        )
        .unwrap();
        assert_eq!(check_block_correctness(&sudoku, 0), false);
    }

    #[test]
    fn block_correct() {
        let sudoku = create_board(
            "123000000\
             456000000\
             789000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000"
                .to_string(),
        )
        .unwrap();
        assert!(check_block_correctness(&sudoku, 0));
        let sudoku = create_board(
            "000000000\
             000000000\
             000000000\
             000123000\
             000456000\
             000789000\
             000000000\
             000000000\
             000000000"
                .to_string(),
        )
        .unwrap();
        assert!(check_block_correctness(&sudoku, 4));
        let sudoku = create_board(
            "000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000000\
             000000123\
             000000456\
             000000789"
                .to_string(),
        )
        .unwrap();
        assert!(check_block_correctness(&sudoku, 8));
    }

    #[test]
    fn sudoku_correct() {
        let sudoku = create_board(
            "123456789\
             456789123\
             789123456\
             234567891\
             567891234\
             891234567\
             345678912\
             678912345\
             912345678"
                .to_string(),
        )
        .unwrap();
        assert!(check_correctness_of_sudoku(&sudoku));
    }
}
