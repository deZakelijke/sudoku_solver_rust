use crate::sudoku::Sudoku;
use std::collections::HashSet;

mod simple_solver;
mod most_restricted_cell_first;

pub enum SolverAlgorithm {
    Simple,
    MostRestricted,
}

impl Sudoku {
    pub fn solve_sudoku(&mut self, algorithm: &SolverAlgorithm) {
        match algorithm {
            SolverAlgorithm::Simple => self.simple_solver(),
            SolverAlgorithm::MostRestricted => self.solve_from_most_restricted_cell(),
        }
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

mod tests {
    use super::*;

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
