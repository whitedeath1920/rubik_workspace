#![allow(dead_code)]
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use cube_core::{moves::{Axis, CubeVect, Face, LayerSpec, Move, MoveKind}, state,n_state};
fn test_n_new(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("new from dimension");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        group.bench_function(format!("dimensions {}", dimension), |f| {
            f.iter(|| {
                black_box(n_state::CubeState::unchecked_new(dimension));
            });
        });
    }

    group.finish();
}
fn test_n_to_vect(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("new to vect dimension");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let cube = n_state::CubeState::unchecked_new(dimension);
        group.bench_function(format!("dimensions {}", dimension), |f| {
            f.iter(|| {
                black_box(cube.to_vec());
            });
        });
    }

    group.finish();
}
fn test_res(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("test res");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let mut c1 = state::CubeState::new(dimension);
        let c2 = state::CubeState::new(dimension);
        group.bench_function(format!("dimensions {}, assigned", dimension), |f| {
            f.iter(|| {
                black_box(c1 += &c2);
            });
        });
        group.bench_function(format!("dimensions {}, cloned", dimension), |f| {
            f.iter(|| {
                black_box(c1.clone() + &c2);
            });
        });
    }

    group.finish();
}
fn test_n_sum(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("test n sum");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let mut c1 = n_state::CubeState::unchecked_new(dimension);
        let c2 = n_state::CubeState::unchecked_new(dimension);
        group.bench_function(format!("dimensions {}, assigned", dimension), |f| {
            f.iter(|| {
                black_box(c1 += &c2);
            });
        });
        group.bench_function(format!("dimensions {}, cloned", dimension), |f| {
            f.iter(|| {
                black_box(c1.clone() + &c2);
            });
        });
    }

    group.finish();
}
fn test_sum(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("test sum");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let mut c1 = state::CubeState::new(dimension);
        let c2 = state::CubeState::new(dimension);
        group.bench_function(format!("dimensions {}, assigned", dimension), |f| {
            f.iter(|| {
                black_box(c1 += &c2);
            });
        });
        group.bench_function(format!("dimensions {}, cloned", dimension), |f| {
            f.iter(|| {
                black_box(c1.clone() + &c2);
            });
        });
    }

    group.finish();
}
fn test_to_vect(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("to vect dimension");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let cube = state::CubeState::new(dimension);
        group.bench_function(format!("dimensions {}", dimension), |f| {
            f.iter(|| {
                black_box(cube.to_vec());
            });
        });
    }

    group.finish();
}
fn test_new(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("new from dimension");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        group.bench_function(format!("dimensions {}", dimension), |f| {
            f.iter(|| {
                black_box(state::CubeState::new(dimension));
            });
        });
    }

    group.finish();
}
fn test_new_vect(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("new Vect from dimension");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        group.bench_function(format!("dimensions {}", dimension), |f| {
            f.iter(|| {
                black_box(CubeVect::new(dimension));
            });
        });
    }

    group.finish();
}
fn test_vect_into_state(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("Vect into State from dimension");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        group.bench_function(format!("dimensions {}", dimension), |f| {
            f.iter(|| {
                let _: state::CubeState = black_box(CubeVect::new(dimension).into());
            });
        });
    }
}
fn test_move_vect(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("move Vect into State from dimension");
    let r = Move {
        kind: MoveKind::FaceTurn {
            face: Face::R,
            layer: LayerSpec::Outer,
        },
        qturns: 1,
    };
    let x = Move {
        kind: MoveKind::Rotation { axis:Axis::X },
        qturns: 1,
    };
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let cube = CubeVect::new(dimension);
        group.bench_function(format!("Basic Move R, dimension {}", dimension), |f| {
            f.iter(|| {
                let _ = black_box(cube.mv(r));
            });
        });
        group.bench_function(format!("Basic Rotation X, dimension {}", dimension), |f| {
            f.iter(|| {
                let _ = black_box(cube.mv(x));
            });
        });
    }

}
fn test_mul(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("test mul");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let c1 = state::CubeState::new(dimension);
        group.bench_function(format!("dimensions {}, cloned", dimension), |f| {
            f.iter(|| {
                black_box(500 * c1.clone());
            });
        });
        group.bench_function(format!("dimensions {}, -cloned", dimension), |f| {
            f.iter(|| {
                black_box(-500 * c1.clone());
            });
        });
    }

    group.finish();
}
fn test_cycle_decomp(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("test cycle decomp");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let c1 = state::CubeState::new(dimension);
        group.bench_function(format!("dimensions {}, cloned", dimension), |f| {
            f.iter(|| {
                black_box(c1.cycle_decomposition());
            });
        });
    }

    group.finish();
}
fn test_check(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("test check");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let c1 = state::CubeState::new(dimension);
        group.bench_function(format!("dimensions {}, cloned", dimension), |f| {
            f.iter(|| {
                let _ = black_box(c1.check());
            });
        });
    }

    group.finish();
}
fn test_modulus(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("test modulus");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let c1 = state::CubeState::new(dimension);
        group.bench_function(format!("dimensions {}, cloned", dimension), |f| {
            f.iter(|| {
                black_box(c1.get_modulus());
            });
        });
    }

    group.finish();
}
fn test_eq(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("eq from dimension");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let cube1 = state::CubeState::new(dimension);
        let cube2 = state::CubeState::new(dimension);
        group.bench_function(format!("dimensions {}", dimension), |f| {
            f.iter(|| {
                black_box(cube1 == cube2);
            });
        });
    }

    group.finish();
}
fn test_identity(c: &mut Criterion) {
    let dimensions = [2, 3, 4, 100, 1_000];
    let mut group = c.benchmark_group("identity from dimension");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(200);
    for dimension in dimensions {
        let cube1 = state::CubeState::new(dimension);
        group.bench_function(format!("dimensions {}", dimension), |f| {
            f.iter(|| {
                black_box(cube1.identity());
            });
        });
    }

    group.finish();
}
criterion_group!(benches, test_n_sum, test_sum);
// criterion_group!(benches, test_n_new, test_new);
// criterion_group!(benches, test_res, test_check,test_cycle_decomp,test_eq,test_identity,test_modulus,test_move_vect,test_mul,test_new,test_new_vect,test_sum,test_to_vect,test_vect_into_state);
// criterion_group!(benches, test_sum, test_res, test_mul);
// criterion_group!(benches, test_identity);
// criterion_group!(benches, test_eq, test_new_vect, test_vect_into_state, test_move_vect, test_to_vect);
// criterion_group!(benches, test_vect_into_state, test_new_vect);
// criterion_group!(benches, test_sum, test_res, test_mul);
// criterion_group!(benches,  test_cycle_decomp, test_check, test_modulus);
criterion_main!(benches);
