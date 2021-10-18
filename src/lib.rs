use thiserror::Error;

#[derive(Error, Debug)]
pub enum SudokuError {
    #[error("unsolvable sudoku")]
    Unsolvable,
}

#[derive(Copy, Clone)]
pub struct Sudoku {
    cells: [[i32; 9]; 9],
}

impl Sudoku {
    pub fn new(cells: [[i32; 9]; 9]) -> Self {
        Self { cells }
    }

    pub fn solve(self: &mut Self) -> Result<(), anyhow::Error> {
        Ok(())
    }
    pub fn print(self: &Self) {}
}
