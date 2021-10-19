use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sudoku_solver::Sudoku;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve easy", |b| {
        b.iter(|| {
            let mut sudoku = Sudoku::new(black_box([
                [5, 3, 0, 0, 7, 0, 0, 0, 0],
                [6, 0, 0, 1, 9, 5, 0, 0, 0],
                [0, 9, 8, 0, 0, 0, 0, 6, 0],
                [8, 0, 0, 0, 6, 0, 0, 0, 3],
                [4, 0, 0, 8, 0, 3, 0, 0, 1],
                [7, 0, 0, 0, 2, 0, 0, 0, 6],
                [0, 6, 0, 0, 0, 0, 2, 8, 0],
                [0, 0, 0, 4, 1, 9, 0, 0, 5],
                [0, 0, 0, 0, 8, 0, 0, 7, 9],
            ]))
            .unwrap();
            sudoku.solve().unwrap();
        })
    });

    c.bench_function("solve evil", |b| {
        b.iter(|| {
            let mut sudoku = Sudoku::new(black_box([
                [0, 5, 6, 9, 0, 0, 0, 7, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 8],
                [4, 0, 0, 0, 0, 0, 0, 0, 0],
                [9, 0, 0, 0, 0, 0, 0, 4, 0],
                [2, 0, 0, 3, 0, 0, 0, 0, 0],
                [0, 4, 3, 0, 0, 8, 0, 0, 7],
                [0, 0, 0, 0, 0, 9, 6, 0, 0],
                [0, 0, 2, 0, 0, 0, 0, 0, 0],
                [0, 3, 7, 6, 0, 0, 0, 5, 0],
            ]))
            .unwrap();
            sudoku.solve().unwrap();
        })
    });

    c.bench_function("solve designed against backtracking", |b| {
        b.iter(|| {
            let mut sudoku = Sudoku::new(black_box([
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 3, 0, 8, 5],
                [0, 0, 1, 0, 2, 0, 0, 0, 0],
                [0, 0, 0, 5, 0, 7, 0, 0, 0],
                [0, 0, 4, 0, 0, 0, 1, 0, 0],
                [0, 9, 0, 0, 0, 0, 0, 0, 0],
                [5, 0, 0, 0, 0, 0, 0, 7, 3],
                [0, 0, 2, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 4, 0, 0, 0, 9],
            ]))
            .unwrap();
            sudoku.solve().unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
