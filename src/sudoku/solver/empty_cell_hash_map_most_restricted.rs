use crate::sudoku::Sudoku;
use std::collections::{HashMap, HashSet};

impl Sudoku {
    /// Solve the sudoku by creating a hash set of all options each cell has instead of
    /// calculating that again and again.
    pub fn solve_with_storing_empty_cell_options(&mut self) {
        let mut possible_value_map = self.create_hash_map_of_all_possible_values();
        let _ = self.pick_values_from_possible_values_map(&mut possible_value_map);
        // Get entry with shortest value
        // Fill in, and cross out this option from other entries in the same row/col
        // Does it make more sense to let the values be a set instead of a vec?
        // We only add or remove something, or check if a char is present.
    }

    fn pick_values_from_possible_values_map(
        &mut self,
        possible_value_map: &mut HashMap<(usize, usize), HashSet<char>>,
    ) -> Result<&Self, ()> {
        if possible_value_map.len() > 0 {
            // println!("There is a possible value");
            let (row, col) = self.get_cell_with_fewest_options(possible_value_map);
            let possible_values = possible_value_map.remove(&(row, col)).unwrap();
            // println!("Possible values ({}, {}):  {:?}", row, col, possible_values);
            for possible_value in possible_values.iter() {
                // println!("Setting value ({}, {}): {}", row, col, possible_value);
                self.board[row][col] = possible_value.clone();
                self.update_possible_value_map_after_selection(
                    possible_value_map,
                    (row, col),
                    possible_value,
                );
                match self.pick_values_from_possible_values_map(possible_value_map) {
                    Ok(_) => {
                        return Ok(self);
                    }
                    Err(_) => {
                        // TODO Put possible value vec back into the hashmap;
                        // TODO Undo update
                        self.board[row][col] = '0';
                    }
                };
            }
            return Err(());
        } else if self.check_sudoku_completed() {
            return Ok(self);
        }
        Err(())
    }

    fn create_hash_map_of_all_possible_values(&self) -> HashMap<(usize, usize), HashSet<char>> {
        let mut possible_value_map = HashMap::new();
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] != '0' {
                    continue;
                }
                let possible_values = self.check_possible_values(i, j);
                if possible_values.len() != 0 {
                    possible_value_map.insert((i, j), HashSet::from_iter(possible_values));
                }
            }
        }
        possible_value_map
    }

    fn get_cell_with_fewest_options(
        &self,
        possible_value_map: &HashMap<(usize, usize), HashSet<char>>,
    ) -> (usize, usize) {
        let mut fewest_option_count = 10;
        let mut cell_with_fewest_possibilities = (0, 0);
        for (cell_index, possibilities) in possible_value_map.iter() {
            if possibilities.len() < fewest_option_count {
                fewest_option_count = possibilities.len();
                cell_with_fewest_possibilities = *cell_index;
            }
        }
        cell_with_fewest_possibilities
    }

    fn update_possible_value_map_after_selection(
        &self,
        possible_value_map: &mut HashMap<(usize, usize), HashSet<char>>,
        cell_index: (usize, usize),
        choice: &char,
    ) {
        let (row, col) = cell_index;
        for i in 0..9 {
            match possible_value_map.get_mut(&(row, i)) {
                Some(value) => value.remove(choice),
                None => false,
            };
            match possible_value_map.get_mut(&(i, col)) {
                Some(value) => value.remove(choice),
                None => false,
            };
            match possible_value_map.get_mut(&(i / 3 + (row / 3) * 3, i % 3 + (col / 3) * 3)) {
                Some(value) => value.remove(choice),
                None => false,
            };
        }
        // We might want to keep track of where we removed something to make
        // backtracking easier
    }
}
