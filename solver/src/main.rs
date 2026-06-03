use std::sync::atomic::{AtomicUsize};

use crate::puzzle::{check_puzzle, setup_puzzle};

mod puzzle;
mod models;

const SIZE_ROWS_COLS: usize = 9;
const SQUARE_ROW_COL: usize = 3;
const TOTAL_CELLS: usize = 81;

static UNSOLVED: AtomicUsize = AtomicUsize::new(81);

fn main() {
    let puzzle = puzzle::create_puzzle();

    let mut sudoku = setup_puzzle(puzzle);

    println!("Original");
    puzzle::print_puzzle(&sudoku.cells);

    check_puzzle(&mut sudoku);
    check_puzzle(&mut sudoku);
    check_puzzle(&mut sudoku);
    check_puzzle(&mut sudoku);
    check_puzzle(&mut sudoku);

    println!("Finished");
    puzzle::print_puzzle(&sudoku.cells);
}
