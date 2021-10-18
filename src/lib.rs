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

    pub fn solve(&mut self) -> Result<(), anyhow::Error> {
        //Err(SudokuError::Unsolvable.into())

        Ok(())
    }
    pub fn print(self) {
        for row in 0..9 {
            // Print horizontal separator
            if row % 3 == 0 && row != 0 {
                println!("------+-------+------");
            }

            for col in 0..9 {
                // Print vertical separator
                if col % 3 == 0 && col != 0 {
                    print!("| ");
                }

                // Print cell
                if self.cells[row][col] == 0 {
                    print!(" ");
                } else {
                    print!("{}", self.cells[row][col]);
                }

                // Print space separator
                if col < 8 {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
