use crate::sudoku::checker;
use crate::sudoku::Sudoku;
use std::collections::HashSet;


impl Sudoku {
    pub fn solve_sudoku(&mut self, algorithm: String) {
        if algorithm == "simple" {
            self.simple_solver();
        }
    }

    fn simple_solver(&mut self) {
        self.fill_value_and_check().unwrap();
    }

    /// Solve the sudoku by findin the first empty cell, trying a value that is not directly
    /// blocked, and then continuing to the next cell. If there are no options for an empty
    /// cell, backtrack and try the next value for the last cell that was filled in.
    /// If all options for a call have been tried, backtrack further and try the next option
    /// for the last cell before that.
    fn fill_value_and_check(&mut self) -> Result<&Self, ()> {
        if checker::check_sudoku_completed(&self) {
            return Ok(self);
        }
        if let Ok((row_index, column_index, options)) = self.choose_first_possible_value() {
            for option in options.iter() {
                self.board[row_index][column_index] = *option;
                match self.fill_value_and_check() {
                    Ok(_) => {
                        return Ok(self);
                    }
                    Err(_) => {
                        self.board[row_index][column_index] = '0';
                    }
                };
            }
            return Err(());
        } else {
            return Err(());
        }
    }

    /// Finds the first possible cell in the sudoku that has not yet been filled
    /// It iterates from left to right, top to bottom
    /// Returns the index of the cell and the values that are possible to place in the
    /// cell based on which values are directly blocked by other cells in the same row,
    /// column, or block.
    fn choose_first_possible_value(&self) -> Result<(usize, usize, Vec<char>), ()> {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == '0' {
                    let possible_values = self.check_possible_values(i, j);
                    if possible_values != [] {
                        return Ok((i, j, possible_values));
                    } else {
                        return Err(());
                    }
                }
            }
        }
        Err(())
    }

    /// Produce a vector of values that could be filled in the cell passed by the index
    /// based on which values are directly blocked by other cells in the same row, column,
    /// or block.
    fn check_possible_values(&self, row: usize, column: usize) -> Vec<char> {
        let mut existing_row_values: HashSet<char> = HashSet::new();
        let mut existing_column_values: HashSet<char> = HashSet::new();
        let mut existing_block_values: HashSet<char> = HashSet::new();
        for i in 0..9 {
            let row_value = self.board[row][i];
            let col_value = self.board[i][column];
            let block_value = self.board[i / 3 + (row / 3) * 3][i % 3 + (column / 3) * 3];
            existing_row_values.insert(row_value.clone());
            existing_column_values.insert(col_value.clone());
            existing_block_values.insert(block_value.clone());
        }
        existing_row_values.extend(&existing_column_values);
        existing_row_values.extend(&existing_block_values);
        let possible_values = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
            .iter()
            .filter(|s| !existing_row_values.contains(s))
            .cloned()
            .collect();
        return possible_values;
    }
}

#[cfg(test)]
mod tests {
    use crate::checker::check_correctness_of_sudoku;
    use crate::sudoku::Sudoku;

    #[test]
    fn fill_empty_sudoku() {
        let mut sudoku = Sudoku::create_board(
            "000000000\
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
        sudoku.fill_value_and_check().unwrap();
        assert!(check_correctness_of_sudoku(&sudoku));
    }

    #[test]
    fn get_index_and_possible_values_of_empty_sudoku() {
        let sudoku = Sudoku::create_board(
            "000000000\
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
        assert_eq!(
            sudoku.choose_first_possible_value().unwrap(),
            (0, 0, vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'])
        );
    }

    #[test]
    fn get_error_when_looking_for_value_to_fill_on_full_suduku() {
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
        assert_eq!(sudoku.choose_first_possible_value(), Err(()));
    }

    #[test]
    fn get_error_when_looking_for_value_in_blocked_cell() {
        let sudoku = Sudoku::create_board(
            "012345678\
             900000000\
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
        assert_eq!(sudoku.choose_first_possible_value(), Err(()));
    }
    #[test]
    fn get_value_when_looking_for_value_in_sudoku() {
        let sudoku = Sudoku::create_board(
            "012345678\
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
        assert_eq!(
            sudoku.choose_first_possible_value().unwrap(),
            (0, 0, vec!['9'])
        );

        let sudoku = Sudoku::create_board(
            "123456789\
             456789123\
             789123456\
             234567891\
             567891234\
             891200067\
             345678912\
             678912345\
             912305678"
                .to_string(),
        )
        .unwrap();
        assert_eq!(
            sudoku.choose_first_possible_value().unwrap(),
            (5, 4, vec!['3', '4'])
        );

        // let sudoku = Sudoku::create_board(
        //     "241736589\
        //      573924106\
        //      800501002\
        //      700295018\
        //      009400305\
        //      652800007\
        //      465080071\
        //      000159004\
        //      908007053"
        //         .to_string(),
        // )
        // .unwrap();
        // assert_eq!(
        //     choose_first_possible_value(&sudoku).unwrap(),
        //     (1, 7, vec!['8', '4'])
        // );
    }

    #[test]
    fn get_possible_values_of_empty_sudoku() {
        let sudoku = Sudoku::create_board(
            "000000000\
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
        assert_eq!(
            sudoku.check_possible_values(0, 0),
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
        assert_eq!(
            sudoku.check_possible_values(8, 2),
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
        assert_eq!(
            sudoku.check_possible_values(3, 6),
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
        assert_eq!(
            sudoku.check_possible_values(7, 7),
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }

    #[test]
    fn get_possible_values_of_filled_sudoku() {
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
        assert_eq!(sudoku.check_possible_values(0, 0), vec![]);
        assert_eq!(sudoku.check_possible_values(8, 2), vec![]);
        assert_eq!(sudoku.check_possible_values(3, 6), vec![]);
        assert_eq!(sudoku.check_possible_values(7, 7), vec![]);
    }
    #[test]
    fn get_possible_values_for_partiall_filled_row() {
        let sudokus: Vec<Sudoku> = vec![
            Sudoku::create_board(
                "123400000\
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
            .unwrap(),
            Sudoku::create_board(
                "000000000\
                 000000000\
                 000123400\
                 000000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000"
                    .to_string(),
            )
            .unwrap(),
            Sudoku::create_board(
                "000000000\
                 000000000\
                 000000000\
                 000000000\
                 000123400\
                 000000000\
                 000000000\
                 000000000\
                 000000000"
                    .to_string(),
            )
            .unwrap(),
        ];
        for (index, sudoku) in sudokus.iter().enumerate() {
            assert_eq!(
                sudoku.check_possible_values(index * 2, 0),
                vec!['5', '6', '7', '8', '9']
            );
            assert_eq!(
                sudoku.check_possible_values(index * 2, 4),
                vec!['5', '6', '7', '8', '9']
            );
            assert_eq!(
                sudoku.check_possible_values(index * 2, 8),
                vec!['5', '6', '7', '8', '9']
            );
        }
    }

    #[test]
    fn get_possible_values_for_partiall_filled_column() {
        let sudokus: Vec<Sudoku> = vec![
            Sudoku::create_board(
                "100000000\
                 200000000\
                 300000000\
                 400000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000"
                    .to_string(),
            )
            .unwrap(),
            Sudoku::create_board(
                "000000000\
                 000000000\
                 000000000\
                 001000000\
                 002000000\
                 003000000\
                 004000000\
                 000000000\
                 000000000"
                    .to_string(),
            )
            .unwrap(),
            Sudoku::create_board(
                "000000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000\
                 000010000\
                 000020000\
                 000030000\
                 000040000"
                    .to_string(),
            )
            .unwrap(),
        ];
        for (index, sudoku) in sudokus.iter().enumerate() {
            assert_eq!(
                sudoku.check_possible_values( 0, index * 2),
                vec!['5', '6', '7', '8', '9']
            );
            assert_eq!(
                sudoku.check_possible_values(4, index * 2),
                vec!['5', '6', '7', '8', '9']
            );
            assert_eq!(
                sudoku.check_possible_values(8, index * 2),
                vec!['5', '6', '7', '8', '9']
            );
        }
    }
    #[test]
    fn get_possible_values_for_partiall_filled_block() {
        let sudokus: Vec<Sudoku> = vec![
            Sudoku::create_board(
                "120000000\
                 030000000\
                 004000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000"
                    .to_string(),
            )
            .unwrap(),
            Sudoku::create_board(
                "000000000\
                 000000000\
                 000000000\
                 000104000\
                 000003000\
                 000020000\
                 004000000\
                 000000000\
                 000000000"
                    .to_string(),
            )
            .unwrap(),
            Sudoku::create_board(
                "000000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000\
                 000000000\
                 000000100\
                 000000040\
                 000000032"
                    .to_string(),
            )
            .unwrap(),
        ];
        for (index, sudoku) in sudokus.iter().enumerate() {
            assert_eq!(
                sudoku.check_possible_values(index * 3 + 0, index * 3),
                vec!['5', '6', '7', '8', '9']
            );
            assert_eq!(
                sudoku.check_possible_values(index * 3 + 1, index * 3),
                vec!['5', '6', '7', '8', '9']
            );
            assert_eq!(
                sudoku.check_possible_values(index * 3 + 2, index * 3),
                vec!['5', '6', '7', '8', '9']
            );
        }
    }
}
