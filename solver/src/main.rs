use std::sync::atomic::{AtomicUsize, Ordering};

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

    while UNSOLVED.load(Ordering::Relaxed) > 0 {
        let progress = check_puzzle(&mut sudoku);
        if !progress {
            break;
        }
    }
    if UNSOLVED.load(Ordering::Relaxed) > 0 {
        println!("Failed to solve :(");
    } else {
        println!("Solved :)");
    }
    puzzle::print_puzzle(&sudoku.cells);
}
