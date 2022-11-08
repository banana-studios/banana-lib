use banana_lib::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;

const WIDTH: usize = 300;
const HEIGHT: usize = 200;
const DIFFICULTIES: [i32; 4] = [1, 5, 10, 100];

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl GridPoint for Point {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}

/// Some hard math operation to apply to the grid.
// fn operation(difficulty: i32) -> impl Fn(Point) -> i32 {
//     let f = black_box(|difficulty| {
//         move |Point { mut x, mut y }| {
//             for _ in 0..difficulty {
//                 y = x + y;
//                 x = if x % 2 == 0 { 1 } else { x + y }
//             }
//             x + y
//         }
//     });

//     f(difficulty)
// }

// fn to_point(i: usize) -> Point { Point { x: (i % WIDTH) as i32, y: (i / WIDTH) as i32 } }

/// Benchmark the `get` method of Gridlike with random access.
fn get_grid_bench_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("GetRandom");
    let point = || Point {
        x: rand::thread_rng().gen_range(0..WIDTH as i32),
        y: rand::thread_rng().gen_range(0..HEIGHT as i32),
    };

    for d in &DIFFICULTIES {
        group.bench_with_input(BenchmarkId::new("1D Vec", d), &d, |b, d| {
            let g = Grid::new((WIDTH, HEIGHT), **d);
            b.iter(|| {
                for _ in 0..**d {
                    black_box(g.get(point()));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("2D Vec", d), &d, |b, d| {
            let g = Grid2D::new((WIDTH, HEIGHT), **d);
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
    for d in &DIFFICULTIES {
        group.bench_with_input(BenchmarkId::new("1D Vec", d), &d, |b, d| {
            let g = Grid::new((WIDTH, HEIGHT), d);
            b.iter(|| {
                for x in 0..**d {
                    for y in 0..**d {
                        black_box(g.get((x, y)));
                    }
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("2D Vec", d), &d, |b, d| {
            let g = Grid2D::new((WIDTH, HEIGHT), d);
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

criterion_group!(benches, get_grid_bench_order, get_grid_bench_random);
criterion_main!(benches);
