use std::sync::atomic::{Ordering};
use crate::UNSOLVED;

#[derive(Clone)]
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
    if cell.val != 0 { return; }

    let mut found: Option<usize> = None;
    for (i, &p) in cell.possible.iter().enumerate() {
        if p == 1 {
            if found.is_some() {
                // more than one candidate; can't solve here
                found = None;
                break;
            } else {
                found = Some(i);
            }
        }
    }

    if let Some(k) = found {
        cell.val = (k + 1) as i32;
        cell.solvable = 0;
        UNSOLVED.fetch_sub(1, Ordering::Relaxed);
    }
}