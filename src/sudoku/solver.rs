use crate::sudoku::checker;
use crate::sudoku::Sudoku;
use std::collections::HashSet;
use std::error::Error;

pub fn solve_sudoku(sudoku: &mut Sudoku, algorithm: String) {
    if algorithm == "simple" {
        simple_solver(sudoku);
    }
}

fn simple_solver(sudoku: &mut Sudoku) {
    fill_value_and_check(sudoku).unwrap();
}

fn fill_value_and_check(sudoku: &mut Sudoku) -> Result<&Sudoku, Box<dyn Error>> {
    if let Ok((row_index, column_index, options)) = choose_first_possible_value(sudoku) {
        for option in options.iter() {
            sudoku.board[row_index][column_index] = *option;
            if checker::check_correctness_of_sudoku(sudoku) {
                match fill_value_and_check(sudoku) {
                    Ok(_) => break,
                    Err(_) => {
                        sudoku.board[row_index][column_index] = '0';
                    }
                };
            }
        }
        Ok(sudoku)
    } else {
        Ok(sudoku)
    }
}

/// Finds the first possible cell in the sudoku that has not yet been filled
/// It iterates from left to right, top to bottom
/// Returns the index of the cell and the values that are possible to place in the
/// cell based on which values are directly blocked by other cells in the same row,
/// column, or block.
fn choose_first_possible_value(sudoku: &Sudoku) -> Result<(usize, usize, Vec<char>), ()> {
    for i in 0..sudoku.board.len() {
        for j in 0..sudoku.board[i].len() {
            if sudoku.board[i][j] == '0' {
                let possible_values = check_possible_values(sudoku, i, j);
                return Ok((i, j, possible_values));
            }
        }
    }
    Err(())
}

/// Produce a vector of values that could be filled in the cell passed by the index
/// based on which values are directly blocked by other cells in the same row, column,
/// or block.
fn check_possible_values(sudoku: &Sudoku, row: usize, column: usize) -> Vec<char> {
    let mut existing_row_values: HashSet<char> = HashSet::from_iter(sudoku.board[row].clone());
    let existing_column_values: HashSet<char> =
        HashSet::from_iter(sudoku.board[0..9][column].clone());
    let mut existing_block_values: HashSet<char> = HashSet::new();
    for i in 0..9 {
        let value = sudoku.board[i / 3 + row / 3][i % 3 + column % 3];
        existing_block_values.insert(value.clone());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sudoku::create_board;

    #[test]
    fn get_possible_values_of_empty_sudoku() {
        let sudoku = create_board(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        )
        .unwrap();
        assert_eq!(
            check_possible_values(&sudoku, 0, 0),
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
        assert_eq!(
            check_possible_values(&sudoku, 8, 2),
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
        assert_eq!(
            check_possible_values(&sudoku, 3, 6),
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
        assert_eq!(
            check_possible_values(&sudoku, 7, 7),
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }

    #[test]
    fn get_possible_values_of_filled_sudoku() {
        let sudoku = create_board(
            "123456789456789123789123456234567891567891234891234567345678912678912345912345678"
                .to_string(),
        )
        .unwrap();
        assert_eq!(check_possible_values(&sudoku, 0, 0), vec![]);
        assert_eq!(check_possible_values(&sudoku, 8, 2), vec![]);
        assert_eq!(check_possible_values(&sudoku, 3, 6), vec![]);
        assert_eq!(check_possible_values(&sudoku, 7, 7), vec![]);
    }
}
