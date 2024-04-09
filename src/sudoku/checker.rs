use crate::sudoku::Sudoku;

impl Sudoku {
    fn check_block_correctness(&self, index: usize) -> bool {
        let mut block = Vec::new();
        for i in 0..9 {
            block.push(self.board[i % 3 + (index / 3) * 3][i / 3 + (index % 3) * 3]);
        }
        let dups = Sudoku::check_duplicates_in_slice(&block);
        !dups
    }
    fn check_row_correctness(&self, index: usize) -> bool {
        let row = &self.board[index];
        let dups = Sudoku::check_duplicates_in_slice(&row);
        !dups
    }
    fn check_column_correctness(&self, index: usize) -> bool {
        let mut column = Vec::new();
        for i in 0..self.board.len() {
            column.push(self.board[i][index]);
        }
        let dups = Sudoku::check_duplicates_in_slice(&column);
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

    pub fn check_correctness_of_sudoku(&self) -> bool {
        for index in 0..9 {
            if !self.check_row_correctness(index) {
                return false;
            }
            if !self.check_block_correctness(index) {
                return false;
            }
            if !self.check_column_correctness(index) {
                return false;
            }
        }
        true
    }

    pub fn check_sudoku_is_filled(&self) -> bool {
        for row in self.board.iter() {
            if row.contains(&'0') {
                return false;
            }
        }
        true
    }

    pub fn check_sudoku_completed(&self) -> bool {
        self.check_sudoku_is_filled() && self.check_correctness_of_sudoku()
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::Sudoku;

    #[test]
    fn confirm_find_duplicates() {
        let vec_with_duplicates = vec!['a', 'b', 'b'];
        assert!(Sudoku::check_duplicates_in_slice(&vec_with_duplicates));
    }

    #[test]
    fn confirm_no_duplicates() {
        let vec_without_duplicates = vec!['a', 'b', 'c'];
        assert_eq!(Sudoku::check_duplicates_in_slice(&vec_without_duplicates), false);
    }

    #[test]
    fn row_not_correct() {
        let sudoku = Sudoku::create_board(
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
        assert_eq!(Sudoku::check_row_correctness(&sudoku, 0), false);
    }

    #[test]
    fn row_correct() {
        let sudoku = Sudoku::create_board(
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
        assert!(Sudoku::check_row_correctness(&sudoku, 0));
    }

    #[test]
    fn column_not_correct() {
        let sudoku = Sudoku::create_board(
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
        assert_eq!(Sudoku::check_column_correctness(&sudoku, 0), false);
    }

    #[test]
    fn column_correct() {
        let sudoku = Sudoku::create_board(
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
        assert!(Sudoku::check_column_correctness(&sudoku, 0));
    }

    #[test]
    fn block_not_correct() {
        let sudoku = Sudoku::create_board(
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
        assert_eq!(Sudoku::check_block_correctness(&sudoku, 0), false);
    }

    #[test]
    fn block_correct() {
        let sudoku = Sudoku::create_board(
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
        assert!(Sudoku::check_block_correctness(&sudoku, 0));
        let sudoku = Sudoku::create_board(
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
        assert!(Sudoku::check_block_correctness(&sudoku, 4));
        let sudoku = Sudoku::create_board(
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
        assert!(Sudoku::check_block_correctness(&sudoku, 8));
    }

    #[test]
    fn sudoku_correct() {
        let sudoku = Sudoku::create_board(
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
        assert!(Sudoku::check_correctness_of_sudoku(&sudoku));
    }
}
