use std::sync::atomic::Ordering;

use crate::{SIZE_ROWS_COLS, UNSOLVED, models::{Cell, sudoku::Sudoku}, puzzle::update_sudoku};
pub struct Square {
    cell_idx: Vec<usize>,
    vals: i32,
    possible: [i32; 9],
    solvable: i32,
    next: Option<Box<Square>>,
}

pub fn create_squares() -> Vec<Square> {
    let mut squares = Vec::with_capacity(SIZE_ROWS_COLS);
    for _ in 0..SIZE_ROWS_COLS {
        let square = Square {
            cell_idx: Vec::new(),
            vals: 0,
            solvable: SIZE_ROWS_COLS as i32,
            possible: [1; SIZE_ROWS_COLS],
            next: None,
        };
        squares.push(square);
    }
    squares
}

pub fn populate_squares(squares: &mut [Square], cells: &Vec<Cell>) {
    for (i, cell) in cells.iter().enumerate() {
        let idx = cell.box_index;
        squares[idx].cell_idx.push(i);
    }
}

pub fn update_squares(squares: &mut [Square], cells: &Vec<Cell>, changed: &[usize]) {
    if changed.is_empty() { return; }

    let mut mark = vec![false; SIZE_ROWS_COLS];
    for &ci in changed {
        if ci < cells.len() {
            mark[cells[ci].box_index] = true;
        }
    }

    for sq_idx in 0..SIZE_ROWS_COLS {
        if !mark[sq_idx] { continue; }
        let sq = &mut squares[sq_idx];

        // recompute vals: count of filled cells
        sq.vals = sq.cell_idx.iter().filter(|&&ci| cells[ci].val != 0).count() as i32;

        // recompute possible counts per digit
        for k in 0..SIZE_ROWS_COLS {
            let mut cnt = 0i32;
            for &ci in &sq.cell_idx {
                if cells[ci].possible[k] == 1 { cnt += 1; }
            }
            sq.possible[k] = cnt;
        }

        // recompute solvable aggregate (sum of cell.solvable)
        sq.solvable = sq.cell_idx.iter().map(|&ci| cells[ci].solvable).sum();
    }
}

pub fn single_candidates(sudoku: &mut Sudoku) -> i32 {
 let mut assignments: Vec<(usize, i32)> = Vec::new();

    // scan each square
    for sq_idx in 0..sudoku.squares.len() {
        // clone idx to prevent borrowing conflicts
        let idx = sudoku.squares[sq_idx].cell_idx.clone();

        // loop through possible digits
        for digit in 0..SIZE_ROWS_COLS {
            let mut count = 0usize;
            let mut last_idx = 0usize;

            for &cell_idx in &idx {
                if sudoku.cells[cell_idx].val != 0 { continue; }
                if sudoku.cells[cell_idx].possible[digit] == 1 {
                    count += 1;
                    last_idx = cell_idx;
                    if count > 1 { break; }
                }
            }

            if count == 1 {
                // record assignment, set cell last_idx to digit+1
                assignments.push((last_idx, (digit + 1) as i32));
            }
        }
    }

    // apply assignments and update sudoku
    for (cell_idx, val) in assignments {
        if sudoku.cells[cell_idx].val == 0 {
            sudoku.cells[cell_idx].val = val;
            UNSOLVED.fetch_sub(1, Ordering::Relaxed);
            sudoku.cells[cell_idx].solvable = 0;

            let row = sudoku.cells[cell_idx].row;
            let col = sudoku.cells[cell_idx].col;
            let changed = update_sudoku(&mut sudoku.cells, row, col);
            update_squares(&mut sudoku.squares, &sudoku.cells, &changed);
            return 1;
        }
    }

    0
}
