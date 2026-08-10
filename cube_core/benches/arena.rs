
#![allow(dead_code)]

use criterion::{
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
};

use cube_core::{CubeArena, arena::{Piece24, add_perm, sub_perm}};
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::{hint::black_box, time::Duration};

const DIMS: [usize; 12] = [2,3,4,5,8,16,32,64,96,128,192,255];
// const DIMS: [usize; 6] = [4,5,6,10,11,12];

fn bench_random_cube(c: &mut Criterion) {

    let mut group = c.benchmark_group("random_cube");

    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(100);

    for n in DIMS {

        let mut arena = CubeArena::new_arena(n as u8, 1);

        let mut rng = SmallRng::seed_from_u64(9999);

        group.bench_with_input(
            BenchmarkId::new("randomize", n),
            &n,
            |b, &_n| {

                b.iter(|| {

                    black_box(arena.random_cube(0, &mut rng));

                });
            },
        );
    }

    group.finish();
}

fn bench_check_cube(c: &mut Criterion) {

    let mut group = c.benchmark_group("check_cube");

    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(100);

    for n in DIMS {

        let mut arena = CubeArena::new_arena(n as u8, 1);

        let mut rng = SmallRng::seed_from_u64(7777);

        arena.random_cube(0, &mut rng);

        group.bench_with_input(
            BenchmarkId::new("check", n),
            &n,
            |b, &_n| {

                b.iter(|| {

                    let _ = black_box(arena.is_solvable(0));

                });
            },
        );
    }

    group.finish();
}

fn bench_randomized_add_sub(c: &mut Criterion) {
    let mut group = c.benchmark_group("randomized_add_sub");

    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(100);

    const CUBES: usize = 10_000;

    for n in DIMS {

        let mut arena = CubeArena::new_arena(n as u8, CUBES + 2);

        let mut rng = SmallRng::seed_from_u64(12345);

        for i in 0..CUBES {
            arena.random_cube(i, &mut rng);
        }

        let indices: Vec<(usize, usize)> =
            (0..100_000)
                .map(|_| {
                    (
                        rng.random_range(0..CUBES),
                        rng.random_range(0..CUBES),
                    )
                })
                .collect();

        group.bench_with_input(
            BenchmarkId::new("random_add", n),
            &n,
            |b, &_n| {

                let mut idx = 0usize;

                b.iter(|| {

                    let (a,b_) = indices[idx];

                    black_box(arena.add(a,b_,CUBES));

                    idx += 1;

                    if idx >= indices.len() {
                        idx = 0;
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("random_sub", n),
            &n,
            |b, &_n| {

                let mut idx = 0usize;

                b.iter(|| {

                    let (a,b_) = indices[idx];

                    black_box(arena.sub(a,b_,CUBES + 1));

                    idx += 1;

                    if idx >= indices.len() {
                        idx = 0;
                    }
                });
            },
        );
    }

    group.finish();
}


fn bench_add_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_scaling");

    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(100);

    for n in DIMS {
        let mut arena = CubeArena::new_arena(n as u8, 3);

        let stride = arena.stride() as usize;

        group.bench_with_input(
            BenchmarkId::new("add", n),
            &stride,
            |b, &_stride| {
                b.iter(|| {
                    black_box(arena.add(0,1,2));
                });
            },
        );
    }

    group.finish();
}

fn bench_sub_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("sub_scaling");

    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(100);

    for n in DIMS {
        let mut arena = CubeArena::new_arena(n as u8, 3);

        let stride = arena.stride() as usize;

        group.bench_with_input(
            BenchmarkId::new("sub", n),
            &stride,
            |b, &_stride| {
                b.iter(|| {
                    black_box(arena.sub(0,1,2));
                });
            },
        );
    }

    group.finish();
}


fn bench_aliasing(c: &mut Criterion) {
    let mut group = c.benchmark_group("aliasing");

    let mut arena = CubeArena::new_arena(255, 3);

    group.bench_function("no_alias", |b| {
        b.iter(|| {
            black_box(arena.add(0,1,2));
        });
    });

    group.bench_function("partial_alias", |b| {
        b.iter(|| {
            black_box(arena.add(0,1,1));
        });
    });

    group.bench_function("full_alias", |b| {
        b.iter(|| {
            black_box(arena.add(0,0,0));
        });
    });

    group.finish();
}


fn bench_sequential_vs_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_behavior");

    let cubes = 100_000usize;
    let mut arena = CubeArena::new_arena(32, cubes);

    let mut rng = SmallRng::seed_from_u64(1234);

    let random_indices: Vec<(usize,usize,usize)> =
        (0..cubes)
            .map(|_| {
                (
                    rng.random_range(0..cubes),
                    rng.random_range(0..cubes),
                    rng.random_range(0..cubes),
                )
            })
            .collect();

    group.bench_function("sequential", |b| {
        let mut i = 0usize;

        b.iter(|| {
            let a = i % (cubes - 2);

            black_box(arena.add(a, a+1, a+2));

            i += 1;
        });
    });

    group.bench_function("random", |b| {
        let mut i = 0usize;

        b.iter(|| {
            let (a,b_,c_) = random_indices[i % random_indices.len()];

            black_box(arena.add(a,b_,c_));

            i += 1;
        });
    });

    group.finish();
}


fn bench_cache_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_size");

    let mut tiny = CubeArena::new_arena(255, 3);

    let mut huge = CubeArena::new_arena(255, 10_000);

    group.bench_function("tiny_arena", |b| {
        b.iter(|| {
            black_box(tiny.add(0,1,2));
        });
    });

    let mut rng = SmallRng::seed_from_u64(999);

    group.bench_function("huge_arena_random", |b| {
        b.iter(|| {
            let a = rng.random_range(0..9_997);

            black_box(huge.add(a, a+1, a+2));
        });
    });

    group.finish();
}




fn bench_add24(c: &mut Criterion) {
    let mut group = c.benchmark_group("add24");

    let a = black_box(984818244535754528103549039458486304u128);
    let b = black_box(984818244535754528103549039458486304u128);

    group.bench_function("add24_only", |bench| {
        bench.iter(|| {
            black_box(add_perm::<Piece24>(a,b));
        });
    });

    group.bench_function("sub24_only", |bench| {
        bench.iter(|| {
            black_box(sub_perm::<Piece24>(a,b));
        });
    });

    group.finish();
}


fn bench_bulk_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk");

    let cubes = 100_000usize;

    let mut arena = CubeArena::new_arena(32, cubes);

    group.bench_function("bulk_add", |b| {
        b.iter(|| {
            for i in 0..(cubes - 2) {
                black_box(arena.add(i, i+1, i+2));
            }
        });
    });

    group.finish();
}






criterion_group!(
    benches,
    bench_add_scaling,
    bench_sub_scaling,
    bench_aliasing,
    bench_sequential_vs_random,
    bench_cache_size,
    bench_add24,
    bench_bulk_throughput,

    bench_randomized_add_sub,
    bench_random_cube,
    bench_check_cube,
);

criterion_main!(benches);