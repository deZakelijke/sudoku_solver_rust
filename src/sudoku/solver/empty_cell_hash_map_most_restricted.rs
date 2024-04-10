use crate::sudoku::Sudoku;
use std::collections::{HashMap, HashSet};

impl Sudoku {
    /// Solve the sudoku by creating a hash set of all options each cell has instead of
    /// calculating that again and again.
    pub fn solve_with_storing_empty_cell_options(&mut self) {
        let possible_value_map = self.create_hash_map_of_all_possible_values();
        // Get entry with shortest value
        // Fill in, and cross out this option from other entries in the same row/col
        // Does it make more sense to let the values be a set instead of a vec?
        // We only add or remove something, or check if a char is present.
    }

    fn create_hash_map_of_all_possible_values(&self) -> HashMap<(usize, usize), HashSet<char>> {
        let mut possible_value_map = HashMap::new();
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                let possible_values = self.check_possible_values(i, j);
                possible_value_map.insert((i, j), HashSet::from_iter(possible_values));
            } }
        possible_value_map
    }
}
