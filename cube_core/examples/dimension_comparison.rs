//! # Cube Dimension Comparison
//!
//! Demonstrates how cube operations scale across dimensions n=2..7.
//! Shows the relationship between cube dimension, number of cubies,
//! arena stride, and total moves in a full moveset.
//!
//! Useful for estimating memory and time requirements when building
//! solvers for higher-order cubes (4×4, 5×5, 6×6, 7×7).
//!
//! Usage: `cargo run --example dimension_comparison`

use cube_core::{
    CubeArena, CubeVect,
    cube_moves::{MoveFamily, MoveSet, Turn},
};
use std::time::Instant;

fn main() {
    println!(
        "{0: <4} {1: <12} {2: <10} {3: <12} {4: <10} {5: <18}",
        "n", "cubies", "stride", "orbits", "full_moves", "mem/cube"
    );
    println!(
        "{:-<4} {:-<12} {:-<10} {:-<12} {:-<10} {:-<18}",
        "", "", "", "", "", ""
    );

    for n in 2u8..=7 {
        // ── Physical properties ─────────────────────────────────────
        let cube = CubeVect::new(n as usize);
        let cubies = cube.data().len();

        let arena = CubeArena::new_arena(n, 1);
        let stride = arena.stride();
        let bytes_per_cube = stride as usize * 16; // 16 bytes per u128

        // ── Full moveset size ───────────────────────────────────────
        let full_rule = MoveSet::new(
            vec![
                MoveFamily::Outer,
                MoveFamily::Rotation,
                MoveFamily::Inner,
                MoveFamily::Wide,
            ],
            vec![Turn::Anticlockwise, Turn::Clockwise, Turn::Double],
        );
        let full_moves = full_rule.total_moves(n as usize);

        // ── Orbit count ─────────────────────────────────────────────
        let n_mod_2 = (n & 1) as usize;
        let tmp = (n as usize - 2 - n_mod_2) / 2;
        let num_orbits = 1 + n_mod_2 * 2 + tmp * tmp + tmp + tmp * n_mod_2;

        println!(
            "{n: <4} {cubies: <12} {stride: <10} {num_orbits: <12} {full_moves: <10} {bytes_per_cube: <18}",
            n = n,
            cubies = cubies,
            stride = stride,
            num_orbits = num_orbits,
            full_moves = full_moves,
            bytes_per_cube = bytes_per_cube
        );
    }

    println!();
    println!("=== Operation Timing Across Dimensions ===");

    // ── Time basic operations across dimensions ─────────────────────
    let rng = &mut rand::rng();
    for n in [2u8, 3, 5, 7, 10, 15] {
        let mut arena = CubeArena::new_arena(n, 3);

        // Time: identity generation
        let start = Instant::now();
        for _ in 0..1_000_000 {
            arena.identity(0);
        }
        let id_time = start.elapsed() / 1_000_000;

        // Time: random cube generation
        let start = Instant::now();
        for _ in 0..1_000 {
            arena.random_cube(0, rng);
        }
        let rand_time = start.elapsed() / 1_000;

        // Time: solvability check
        arena.random_cube(0, rng);
        let start = Instant::now();
        for _ in 0..10_000 {
            let _ = arena.is_solvable_slice(arena.get_cube(0));
        }
        let check_time = start.elapsed() / 10_000;

        // Time: add operation
        arena.random_cube(1, rng);
        let start = Instant::now();
        for _ in 0..1_000_000 {
            arena.sub(0, 1, 2);
        }
        let add_time = start.elapsed() / 1_000_000;

        println!(
            "  n={n: >2}: identity={id_time:?}  random={rand_time:?}  check_solvable={check_time:?}  add={add_time:?}"
        );
    }
}
