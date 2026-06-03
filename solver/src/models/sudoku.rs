use crate::models::{Cell, square::Square};

pub struct Sudoku {
    pub cells: Vec<Cell>,
    pub squares: Vec<Square>
}

pub fn create_sudoku(cells: Vec<Cell>, squares: Vec<Square>) -> Sudoku {
    Sudoku {
        cells: cells,
        squares: squares,
    }
}