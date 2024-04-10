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
