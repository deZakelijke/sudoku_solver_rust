use crate::sudoku::Sudoku;

impl Sudoku {
    pub fn simple_solver(&mut self) {
        self.fill_value_and_check().unwrap();
    }

    /// Solve the sudoku by findin the first empty cell, trying a value that is not directly
    /// blocked, and then continuing to the next cell. If there are no options for an empty
    /// cell, backtrack and try the next value for the last cell that was filled in.
    /// If all options for a call have been tried, backtrack further and try the next option
    /// for the last cell before that.
    fn fill_value_and_check(&mut self) -> Result<&Self, ()> {
        if self.check_sudoku_completed() {
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

}

#[cfg(test)]
mod tests {
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
        assert!(sudoku.check_correctness_of_sudoku());
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
    }

}
