use banana_lib::prelude::*;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::*;
const WIDTH: usize = 300;
const HEIGHT: usize = 200;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl GridPoint for Point {
    fn x(&self) -> i32 {
        self.x as i32
    }

    fn y(&self) -> i32 {
        self.y as i32
    }
}

/// Some hard math operation to apply to the grid.
fn operation(difficulty: usize) -> impl Fn(Point) -> usize {
    let f = black_box(|difficulty| {
        move |Point { mut x, mut y }| {
            for _ in 0..difficulty {
                y = x + y;
                x = if x % 2 == 0 { x.pow(2) + y } else { x + y }
            }
            x + y
        }
    });

    f(difficulty)
}

/// Benchmark the `set_all_parallel` method of Gridlike.
fn set_grid_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Set");
    let difficulties = vec![1, 5, 10, 100, 1000, 10000];

    for d in difficulties {
        group.bench_with_input(BenchmarkId::new("1D Vec", d), &d, |b, d| {
            let mut g = Grid2D::new((WIDTH, HEIGHT), *d);
            b.iter(|| {
                let setter = operation(d.clone());
                for (i, item) in g.iter_mut().enumerate() {
                    *item = 1 as usize;
                    *item = setter(Point { x: i % WIDTH, y: i / WIDTH });
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("2D Vec", d), &d, |b, d| {
            let mut g = Grid::new((WIDTH, HEIGHT), *d);
            b.iter(|| {
                let setter = operation(d.clone());
                for (i, item) in g.iter_mut().enumerate() {
                    *item = 1 as usize;
                    *item = setter(Point { x: i % WIDTH, y: i / WIDTH });
                }
            });
        });
    }
    group.finish();
}

/// Benchmark the `get` method of Gridlike with random access.
fn get_grid_bench_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("GetRandom");
    let point = || Point {
        x: rand::thread_rng().gen_range(0..WIDTH),
        y: rand::thread_rng().gen_range(0..HEIGHT),
    };

    for d in &[1, 10, 100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::new("1D Vec", d), &d, |b, d| {
            let mut g = Grid::new((WIDTH, HEIGHT), d);
            b.iter(|| {
                for _ in 0..**d {
                    black_box(g.get(point()));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("2D Vec", d), &d, |b, d| {
            let mut g = Grid2D::new((WIDTH, HEIGHT), d);
            b.iter(|| {
                for _ in 0..**d {
                    black_box(g.get(point()));
                }
            });
        });
    }
    group.finish();
}
/// Benchmark the `get` method of Gridlike, accessing elements in a predictable order.
fn get_grid_bench_order(c: &mut Criterion) {
    let mut group = c.benchmark_group("GetOrder");
    for d in &[1, 50, 100, 200, 1000, 10000] {
        group.bench_with_input(BenchmarkId::new("1D Vec", d), &d, |b, d| {
            let mut g = Grid::new((WIDTH, HEIGHT), d);
            b.iter(|| {
                for x in 0..**d {
                    for y in 0..**d {
                        black_box(g.get((x, y)));
                    }
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("2D Vec", d), &d, |b, d| {
            let mut g = Grid2D::new((WIDTH, HEIGHT), d);
            b.iter(|| {
                for x in 0..**d {
                    for y in 0..**d {
                        black_box(g.get((x, y)));
                    }
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, set_grid_bench, get_grid_bench_order, get_grid_bench_random);
criterion_main!(benches);
