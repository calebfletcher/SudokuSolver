use thiserror::Error;

#[derive(Error, Debug)]
pub enum SudokuError {
    #[error("invalid sudoku")]
    Invalid,
    #[error("unsolvable sudoku")]
    Unsolvable,
    #[error("outside sudoku bounds")]
    OutsideBounds,
    #[error("conflicting cells")]
    ConflictingCells,
}

#[derive(Copy, Clone, Debug)]
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

                // Check for cells with values that are too high
                if cell > 9 {
                    return Err(SudokuError::Invalid);
                }

                // Skip empty cells
                if cell == 0 {
                    continue;
                }

                let cell_box = Self::get_cell_box(row, col)?;

                let mask = 1 << cell;
                // Check if cell is valid compared to the rest of the grid
                if (rows[row] | cols[col] | boxes[cell_box]) & mask != 0 {
                    return Err(SudokuError::Invalid);
                }

                // Update map of values
                rows[row] |= mask;
                cols[col] |= mask;
                boxes[cell_box] |= mask;
            }
        }

        Ok(Self {
            cells,
            rows,
            cols,
            boxes,
        })
    }

    fn get_cell_box(row: usize, col: usize) -> Result<usize, SudokuError> {
        if row >= 9 || col >= 9 {
            return Err(SudokuError::OutsideBounds);
        }

        Ok(3 * (row / 3) + col / 3)
    }

    fn can_put_cell(&self, row: usize, col: usize, value: u32) -> bool {
        // Check row
        if self.rows[row] & 1 << value != 0 {
            return false;
        }

        // Check col
        if self.cols[col] & 1 << value != 0 {
            return false;
        }

        // Check box
        let cell_box = match Self::get_cell_box(row, col) {
            Ok(cell_box) => cell_box,
            Err(_) => return false,
        };
        if self.boxes[cell_box] & 1 << value != 0 {
            return false;
        }

        true
    }

    fn try_put_cell(&mut self, row: usize, col: usize, value: u32) -> Result<(), SudokuError> {
        if value > 9 {
            return Err(SudokuError::Invalid);
        }
        if !self.can_put_cell(row, col, value) {
            return Err(SudokuError::ConflictingCells);
        }

        self.cells[row][col] = value;
        self.rows[row] |= 1 << value;
        self.cols[col] |= 1 << value;

        let cell_box = Self::get_cell_box(row, col)?;
        self.boxes[cell_box] |= 1 << value;

        Ok(())
    }
    fn unset_cell(&mut self, row: usize, col: usize) -> Result<(), SudokuError> {
        let value = self.cells[row][col];
        let mask = !(1 << value);

        self.rows[row] &= mask;
        self.cols[col] &= mask;
        let cell_box = Self::get_cell_box(row, col)?;
        self.boxes[cell_box] &= mask;

        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), SudokuError> {
        for row in 0..9 {
            for col in 0..9 {
                // Check if cell is already filled
                if self.cells[row][col] != 0 {
                    continue;
                }

                // Try each value in turn
                for value in 1..10 {
                    // If value can go in cell
                    if self.try_put_cell(row, col, value).is_ok() {
                        match self.solve() {
                            Ok(_) => {
                                return Ok(());
                            }
                            Err(_) => {
                                self.unset_cell(row, col)?;
                            }
                        }
                    }
                }
                // All values were tried, and none of them worked
                return Err(SudokuError::Unsolvable);
            }
        }
        Ok(())
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
