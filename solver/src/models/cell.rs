use std::sync::atomic::{Ordering};
use crate::UNSOLVED;

pub struct Cell {
    pub val: i32,
    /*
    [1] [1] [1] [1] [1] [1] [1] [1] [1]
    [9] [8] [7] [6] [5] [4] [3] [2] [1]
    array representation of possible numbers for cell
    */
    pub possible: [i32; 9],
    pub solvable: i32,
    pub box_index: usize,
    pub row: usize,
    pub col: usize,
}

pub fn solve_cell(cell: &mut Cell) {
    for i in cell.possible {
        if i == 0 {
            cell.val = i + 1;
            cell.solvable = 0;
            UNSOLVED.fetch_sub(1, Ordering::Relaxed);
        }
    }
}