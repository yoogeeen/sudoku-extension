use crate::{SIZE_ROWS_COLS, models::Cell};
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
