//! # Random Cube Generation & Analysis
//!
//! Demonstrates generating random cube states and computing their
//! algebraic properties: solvability, cycle decomposition, and order.
//! Useful for statistical analysis of the cube group or testing solvers.
//!
//! Usage: `cargo run --example random_generation`

use cube_core::CubeArena;
use std::time::Instant;

fn main() {
    let rng = &mut rand::rng();
    let dimensions = [2u8, 3, 4, 5, 7];

    for &n in &dimensions {
        println!("=== {n}×{n}×{n} Cube ===");

        // ── Create arena with workspace slots ────────────────────────
        // Slot 0: random cube storage
        // Slot 1: workspace for order computation
        let mut arena = CubeArena::new_arena(n, 3);

        // ── Generate a random cube ───────────────────────────────────
        let start = Instant::now();
        arena.random_cube(0, rng);
        let gen_time = start.elapsed();
        println!("  Generation time: {gen_time:?}");

        // ── Check structural validity ────────────────────────────────
        match arena.check_cube(0) {
            Ok(_) => println!("  ✓ Structurally valid"),
            Err(e) => println!("  ✗ Invalid: {e}"),
        }

        // ── Check solvability ────────────────────────────────────────
        match arena.is_solvable(0) {
            Ok(_) => println!("  ✓ Solvable (reachable from solved state)"),
            Err(_) => println!("  ✗ Not solvable — requires parity/orientation fix"),
        }

        // ── Compute cycle decomposition ──────────────────────────────
        let cycles = arena.cycle_decomposition_cube(0);
        let total_cycles: usize = cycles.iter().map(|o| o.len()).sum();
        println!(
            "  Cycle decomposition: {total_cycles} non-trivial cycles across {} orbit families",
            cycles.len()
        );

        // ── Compute cube order ───────────────────────────────────────
        let order = arena.cube_order(0);
        println!("  Cube order: {order} repetitions to return to identity");

        // ── Verify: applying the cube `order` times returns to identity ──
        let start = Instant::now();
        arena.mul(0, order as isize, 1);
        let mul_time = start.elapsed();
        arena.identity(2);
        if arena.get_cube(1) == arena.get_cube(2) {
            println!("  ✓ Verified: {order}×cube = identity (computed in {mul_time:?})");
        }

        println!();
    }
}
