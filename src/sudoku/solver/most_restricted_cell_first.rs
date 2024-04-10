use crate::sudoku::Sudoku;

impl Sudoku {
    pub fn solve_from_most_restricted_cell(&mut self) {
        self.fill_value_and_check_most_restricted().unwrap();
    }

    /// Solves the sudoku by finding the first cell in the sodoku that has one option or
    /// the one that has the fewest available options. Fill it with the first option
    /// found and repeat. If there is a cell that has zero options, backtrack and try
    /// the next value for the last cell that was filled in. 
    /// This approach should be faster than the simple solver because it has to do 
    /// fewer backtracks
    fn fill_value_and_check_most_restricted(&mut self) -> Result<&Self, ()> {
        if self.check_sudoku_completed() {
            return Ok(self);
        }
        if let Ok((row_index, column_index, options)) = self.choose_most_restricted_value() {
            for option in options.iter() {
                self.board[row_index][column_index] = *option;
                match self.fill_value_and_check_most_restricted() {
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


    /// Find the first cell that has just one option, or the first cell that has the 
    /// fewest available options.
    /// It iterates from left to right, top to bottom.
    /// Returns the index of the cell and the values that are possible to place in the
    /// cell based on which values are directly blocked by other cells in the same row,
    /// column, or block.
    fn choose_most_restricted_value(&self) -> Result<(usize, usize, Vec<char>), ()> {
        let mut fewest_options_count = 10;
        let mut fewest_options_values = (10, 10, Vec::new());
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == '0' {
                    let possible_values = self.check_possible_values(i, j);
                    if possible_values.len() == 0 {
                        return Err(());
                    }
                    if possible_values.len() == 1 {
                        return Ok((i, j, possible_values));
                    } 
                    if possible_values.len() < fewest_options_count {
                        fewest_options_count = possible_values.len();
                        fewest_options_values = (i, j, possible_values);
                    }
                }
            }
        }
        if fewest_options_count < 10 {
            return Ok(fewest_options_values);
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
        sudoku.fill_value_and_check_most_restricted().unwrap();
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
            sudoku.choose_most_restricted_value().unwrap(),
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
        assert_eq!(sudoku.choose_most_restricted_value(), Err(()));
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
        assert_eq!(sudoku.choose_most_restricted_value(), Err(()));
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
            sudoku.choose_most_restricted_value().unwrap(),
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
            sudoku.choose_most_restricted_value().unwrap(),
            (5, 5, vec!['4'])
        );
    }
}
