//! # Scramble & Solve Example
//!
//! Demonstrates a real use case: applying a scramble algorithm to a cube,
//! then applying the inverse to verify we return to the solved state.
//! This pattern is essential for puzzle solvers and move validators.
//!
//! Usage: `cargo run --example scramble_and_solve`

use cube_core::{
    CubeArena,
    cube_moves::{MoveFamily, MoveSet, MoveTable, Turn},
    error::Result,
};

fn main() -> Result<()> {
    // ── Setup: build a MoveTable for a 3×3×3 cube with full moveset ──
    let n = 3;
    let rule = MoveSet::new(
        vec![MoveFamily::Outer, MoveFamily::Rotation],
        vec![Turn::Anticlockwise, Turn::Clockwise, Turn::Double],
    );
    let movetable = MoveTable::new(n, &rule)?;

    println!("Built MoveTable for {n}×{n}×{n}");
    println!("  Total moves: {}", movetable.moves().len());
    println!();

    // ── Apply a scramble sequence ────────────────────────────────────
    // A scramble is just a sequence of moves applied to the solved cube.
    let scramble = ["R", "U", "R'", "F2", "U2", "F'", "U", "R", "U2", "R'"];
    let mut arena = CubeArena::new_arena(n as u8, 3);
    // Slot 0: accumulator (starts as identity)
    // Slot 1: temporary workspace

    println!("Scramble: {}", scramble.join(" "));
    for &mv_name in &scramble {
        let move_data = movetable.get_move_s(mv_name);
        // Accumulate: slot0 = slot0 + move_data
        arena.add_slice(0, move_data, 0);
    }
    println!();

    // ── Verify the scrambled state is solvable ───────────────────────
    arena.is_solvable(0).map_err(|e| {
        eprintln!("Error: scrambled cube is not solvable!");
        arena.print_cube(0);
        e
    })?;
    println!("✓ Scrambled state is solvable");
    println!();

    // ── Compute cube order (number of repetitions to return to solved) ──
    let order = arena.cube_order(0);
    println!("Cube order: {order} repetitions to return to solved");
    println!();

    // ── Apply the inverse scramble to return to solved ───────────────
    // Strategy: undo moves in reverse order, using the inverted move
    println!("Applying inverse scramble...");
    for &mv_name in scramble.iter().rev() {
        let mv = movetable.moves_s()[mv_name];
        let inv = mv.invert();
        let inv_name = inv.to_string();
        // Look up inverse by string
        let inv_data = movetable.get_move_s(&inv_name);
        arena.add_slice(0, inv_data, 0);
    }

    // ── Verify we're back to identity ────────────────────────────────
    arena.identity(1); // Reset slot 1 to identity for comparison
    if arena.get_cube(0) == arena.get_cube(1) {
        println!("✓ Inverse scramble returned to solved state!");
    } else {
        eprintln!("✗ Failed to return to solved state");
        arena.print_cube(0);
    }

    Ok(())
}
