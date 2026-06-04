use std::sync::atomic::Ordering;
use std::collections::HashMap;

use crate::{SIZE_ROWS_COLS, UNSOLVED, models::{Cell, square::update_squares, sudoku::Sudoku}, puzzle::update_sudoku};

fn hidden_single_unit(unit: &[usize], sudoku: &mut Sudoku) -> bool {
    let mut assignments: Vec<(usize, i32)> = Vec::new();

    for digit in 0..SIZE_ROWS_COLS {
        let mut count = 0usize;
        let mut last_idx = 0usize;
        for &ci in unit {
            if sudoku.cells[ci].val != 0 { continue; }
            if sudoku.cells[ci].possible[digit] == 1 {
                count += 1;
                last_idx = ci;
                if count > 1 { break; }
            }
        }
        if count == 1 {
            assignments.push((last_idx, (digit + 1) as i32));
        }
    }

    for (cell_idx, val) in assignments {
        if sudoku.cells[cell_idx].val == 0 {
            sudoku.cells[cell_idx].val = val;
            UNSOLVED.fetch_sub(1, Ordering::Relaxed);
            sudoku.cells[cell_idx].solvable = 0;
            let row = sudoku.cells[cell_idx].row;
            let col = sudoku.cells[cell_idx].col;
            let changed = update_sudoku(&mut sudoku.cells, row, col);
            update_squares(&mut sudoku.squares, &sudoku.cells, &changed);
            return true;
        }
    }
    false
}

pub fn scan_hidden_singles(sudoku: &mut Sudoku) -> bool {
    // rows
    for r in 0..SIZE_ROWS_COLS {
        let unit: Vec<usize> = (0..SIZE_ROWS_COLS).map(|c| r * SIZE_ROWS_COLS + c).collect();
        if hidden_single_unit(&unit, sudoku) { return true; }
    }
    // cols
    for c in 0..SIZE_ROWS_COLS {
        let unit: Vec<usize> = (0..SIZE_ROWS_COLS).map(|r| r * SIZE_ROWS_COLS + c).collect();
        if hidden_single_unit(&unit, sudoku) { return true; }
    }
    // boxes
    for b in 0..SIZE_ROWS_COLS {
        let unit = sudoku.squares[b].cell_idx.clone();
        if hidden_single_unit(&unit, sudoku) { return true; }
    }
    false
}

fn mask_from_cell(cell: &Cell) -> u16 {
    let mut m = 0u16;
    for i in 0..SIZE_ROWS_COLS {
        if cell.possible[i] == 1 { m |= 1 << i; }
    }
    m
}

fn scan_naked_pairs_unit(unit: &[usize], sudoku: &mut Sudoku) -> bool {
    let mut by_mask: HashMap<u16, Vec<usize>> = HashMap::new();
    for &ci in unit {
        if sudoku.cells[ci].val != 0 { continue; }
        let m = mask_from_cell(&sudoku.cells[ci]);
        if m.count_ones() == 2 {
            by_mask.entry(m).or_default().push(ci);
        }
    }

    let mut changed = Vec::new();
    for (mask, cells) in by_mask {
        if cells.len() != 2 { continue; }
        for &ci in unit {
            if sudoku.cells[ci].val != 0 { continue; }
            if cells.contains(&ci) { continue; }
            let old_mask = mask_from_cell(&sudoku.cells[ci]);
            let intersect = old_mask & mask;
            if intersect != 0 {
                // remove those bits from possible[]
                for d in 0..SIZE_ROWS_COLS {
                    if (intersect & (1 << d)) != 0 && sudoku.cells[ci].possible[d] == 1 {
                        sudoku.cells[ci].possible[d] = 0;
                        sudoku.cells[ci].solvable -= 1;
                    }
                }
                changed.push(ci);
            }
        }
    }

    if !changed.is_empty() {
        update_squares(&mut sudoku.squares, &sudoku.cells, &changed);
        return true;
    }
    false
}

pub fn scan_naked_pairs(sudoku: &mut Sudoku) -> bool {
    // rows
    for r in 0..SIZE_ROWS_COLS {
        let unit: Vec<usize> = (0..SIZE_ROWS_COLS).map(|c| r * SIZE_ROWS_COLS + c).collect();
        if scan_naked_pairs_unit(&unit, sudoku) { return true; }
    }
    // cols
    for c in 0..SIZE_ROWS_COLS {
        let unit: Vec<usize> = (0..SIZE_ROWS_COLS).map(|r| r * SIZE_ROWS_COLS + c).collect();
        if scan_naked_pairs_unit(&unit, sudoku) { return true; }
    }
    // boxes
    for b in 0..SIZE_ROWS_COLS {
        let unit = sudoku.squares[b].cell_idx.clone();
        if scan_naked_pairs_unit(&unit, sudoku) { return true; }
    }
    false
}
