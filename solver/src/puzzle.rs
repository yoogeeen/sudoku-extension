use crate::{
    SIZE_ROWS_COLS,
    SQUARE_ROW_COL,
    TOTAL_CELLS,
    UNSOLVED,
    models::{
        Cell, 
        solve_cell, 
        square::{create_squares, populate_squares, update_squares}, 
        sudoku::{Sudoku, create_sudoku}}, 
        solving_algos::{scan_hidden_singles, scan_naked_pairs},
};

use std::sync::atomic::Ordering;

pub fn create_puzzle() -> [[i32; SIZE_ROWS_COLS]; SIZE_ROWS_COLS] {
    let mut puzzle: [[i32; SIZE_ROWS_COLS]; SIZE_ROWS_COLS] = [[0; SIZE_ROWS_COLS]; SIZE_ROWS_COLS];
    for &(row, col, val) in &[
        (0, 6, 9),
        (0, 8, 7),
        (1, 0, 6),
        (1, 2, 2),
        (1, 6, 8),
        (2, 7, 3),
        (3, 0, 8),
        (3, 2, 1),
        (3, 3, 4),
        (3, 4, 2),
        (4, 4, 8),
        (4, 6, 7),
        (5, 0, 7),
        (5, 4, 5),
        (5, 6, 2),
        (6, 1, 3),
        (6, 7, 7),
        (7, 1, 6),
        (7, 3, 5),
        (7, 7, 2),
        (7, 8, 1),
        (8, 2, 8),
        (8, 4, 4),
        (8, 5, 3),
    ] {
        puzzle[row][col] = val;
        UNSOLVED.fetch_sub(1, Ordering::Relaxed);
    }

    puzzle

}

pub fn print_puzzle(sudoku: &Vec<Cell>) {
    println!("------------------------------");
    for r in 0..SIZE_ROWS_COLS {
        print!("|");
        for c in 0..SIZE_ROWS_COLS {
            let idx = r * SIZE_ROWS_COLS + c;
            let v = sudoku[idx].val;
            if v == 0 { print!(" . "); } else { print!(" {} ", v); }
            if (c + 1) % 3 == 0 { print!("|"); }
        }
        println!();
        if (r + 1) % 3 == 0 { println!("------------------------------"); }
    }
}

pub fn setup_puzzle(puzzle: [[i32; SIZE_ROWS_COLS]; SIZE_ROWS_COLS]) -> Sudoku {
    let mut cells = Vec::with_capacity(TOTAL_CELLS);

    for i in 0..SIZE_ROWS_COLS {
        for j in 0..SIZE_ROWS_COLS {
            let val = puzzle[i][j];
            let mut cell = Cell {
                val,
                possible: [1; SIZE_ROWS_COLS],
                solvable: SIZE_ROWS_COLS as i32,
                box_index: (i / SQUARE_ROW_COL) * SQUARE_ROW_COL + (j / SQUARE_ROW_COL),
                row: i,
                col: j,
            };
            if val != 0 {
                cell.possible = [0; SIZE_ROWS_COLS];
                cell.solvable = 0;
            }
            cells.push(cell);
        }
    }

    let mut squares = create_squares();
    populate_squares(&mut squares, &cells);

    for idx in 0..cells.len() {
        if cells[idx].val != 0 {
            let row = cells[idx].row;
            let col = cells[idx].col;
            let changed = update_sudoku(&mut cells, row, col);
            update_squares(&mut squares, &cells, &changed);
        }
    }

    create_sudoku(cells, squares)
}

pub fn update_sudoku(cells: &mut Vec<Cell>, row: usize, col: usize) -> Vec<usize> {
    let idx = row * SIZE_ROWS_COLS + col;
    let number = cells[idx].val as usize;
    if number == 0 || number > SIZE_ROWS_COLS { return Vec::new(); }

    // seen vec to prevent double checking
    let mut seen = vec![false; TOTAL_CELLS];

    // row check
    for c in 0..SIZE_ROWS_COLS {
        let p = row * SIZE_ROWS_COLS + c;
        if p != idx { seen[p] = true; }
    }

    // col check
    for r in 0..SIZE_ROWS_COLS {
        let p = r * SIZE_ROWS_COLS + col;
        if p != idx { seen[p] = true; }
    }

    // box check
    let box_row = (row / SQUARE_ROW_COL) * SQUARE_ROW_COL;
    let box_col = (col / SQUARE_ROW_COL) * SQUARE_ROW_COL;
    for r in box_row..box_row + SQUARE_ROW_COL {
        for c in box_col..box_col + SQUARE_ROW_COL {
            let p = r * SIZE_ROWS_COLS + c;
            if p != idx { seen[p] = true; }
        }
    }

    // actual update
    let slot = number - 1;
    let mut changed = Vec::new();
    for p in 0..TOTAL_CELLS {
        if seen[p] && cells[p].possible[slot] == 1 {
            cells[p].possible[slot] = 0;
            cells[p].solvable -= 1;
            changed.push(p);
        }
    }
    changed
}

pub fn check_puzzle(sudoku: &mut Sudoku) -> bool {
    let len = sudoku.cells.len();

    for idx in 0..len {
        if sudoku.cells[idx].solvable == 1 && sudoku.cells[idx].val == 0 {
            {
                let cell = &mut sudoku.cells[idx];
                solve_cell(cell);
            }
            let row = sudoku.cells[idx].row;
            let col = sudoku.cells[idx].col;
            let changed = update_sudoku(&mut sudoku.cells, row, col);
            update_squares(&mut sudoku.squares, &sudoku.cells, &changed);
            return true;
        }
    }

    if scan_hidden_singles(sudoku) { return true; }

    scan_naked_pairs(sudoku)
}
