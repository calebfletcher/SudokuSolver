use thiserror::Error;

#[derive(Error, Debug)]
pub enum SudokuError {
    #[error("invalid sudoku")]
    Invalid,
    #[error("unsolvable sudoku")]
    Unsolvable,
}

#[derive(Copy, Clone)]
pub struct Sudoku {
    cells: [[u32; 9]; 9],
    rows: [u32; 9],
    cols: [u32; 9],
    boxes: [u32; 9],
}

impl Sudoku {
    pub fn new(cells: [[u32; 9]; 9]) -> Result<Self, SudokuError> {
        let mut rows = [0; 9];
        let mut cols = [0; 9];
        let mut boxes = [0; 9];

        for row in 0..9 {
            #[allow(clippy::needless_range_loop)]
            for col in 0..9 {
                let cell = cells[row][col];

                // Check for invalid cells
                if cell > 9 {
                    return Err(SudokuError::Invalid);
                }

                // Skip empty cells
                if cell == 0 {
                    continue;
                }

                let cell_box = 3 * (row / 3) + col / 3;

                // Update map of values
                rows[row] |= 1 << cell;
                cols[col] |= 1 << cell;
                boxes[cell_box] |= 1 << cell;
            }
        }

        Ok(Self {
            cells,
            rows,
            cols,
            boxes,
        })
    }

    pub fn solve(&mut self) -> Result<(), anyhow::Error> {
        //Err(SudokuError::Unsolvable.into())
        self.check();

        Ok(())
    }

    fn check(&self) -> bool {
        // Check rows

        // Check cols

        // Check boxes

        true
    }

    pub fn print(&self) {
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
