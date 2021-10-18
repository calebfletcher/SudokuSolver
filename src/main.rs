use sudoku_solver::Sudoku;

fn main() -> Result<(), anyhow::Error> {
    println!("Solving Sudoku");

    let mut sudoku = Sudoku::new([
        [2, 0, 6, 0, 5, 1, 0, 7, 0],
        [5, 0, 8, 7, 0, 6, 0, 0, 9],
        [4, 0, 0, 0, 0, 0, 0, 0, 1],
        [0, 4, 9, 0, 0, 5, 8, 0, 0],
        [3, 7, 5, 0, 4, 8, 1, 0, 0],
        [8, 2, 0, 3, 0, 9, 7, 0, 5],
        [1, 0, 0, 6, 0, 0, 9, 0, 0],
        [0, 0, 4, 8, 0, 3, 2, 0, 0],
        [7, 0, 2, 0, 0, 0, 0, 0, 3],
    ])?;

    sudoku.solve()?;

    println!("Solved:");
    sudoku.print();
    Ok(())
}
