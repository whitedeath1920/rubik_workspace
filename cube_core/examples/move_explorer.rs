//! # Move Explorer
//!
//! Demonstrates building a complete MoveTable for a custom moveset
//! and exploring all available moves across different cube dimensions.
//! This pattern is useful for building UIs that list legal moves,
//! or for constructing search trees in solving algorithms.
//!
//! Usage: `cargo run --example move_explorer`

use cube_core::cube_moves::{MoveFamily, MoveSet, MoveTable, Turn};

fn main() {
    // ── Define a custom moveset: only Outer face turns + Rotations ──
    let rule = MoveSet::new(
        vec![MoveFamily::Outer, MoveFamily::Rotation],
        vec![Turn::Anticlockwise, Turn::Clockwise, Turn::Double],
    );

    println!("MoveSet configuration:");
    println!(
        "  Outer face turns:  {}",
        if rule.contains_move(MoveFamily::Outer) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  Rotations:         {}",
        if rule.contains_move(MoveFamily::Rotation) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  Inner slice turns: {}",
        if rule.contains_move(MoveFamily::Inner) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  Wide turns:        {}",
        if rule.contains_move(MoveFamily::Wide) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  Clockwise:         {}",
        if rule.contains_turn(Turn::Clockwise) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  Anticlockwise:     {}",
        if rule.contains_turn(Turn::Anticlockwise) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  Double:            {}",
        if rule.contains_turn(Turn::Double) {
            "✓"
        } else {
            "✗"
        }
    );
    println!();

    // ── Explore moves across dimensions ────────────────────────────
    for n in 2..=7 {
        let total = rule.total_moves(n);
        if total == 0 {
            println!("--- {n}×{n}×{n}: no moves available (skipped) ---");
            continue;
        }

        let movetable = MoveTable::new(n, &rule).expect("Failed to build MoveTable");

        println!("--- {n}×{n}×{n}: {total} moves total ---");

        // Group moves by their string representation
        let mut outer_moves: Vec<String> = Vec::new();
        let mut rotation_moves: Vec<String> = Vec::new();

        for (name, _mv) in movetable.moves_s() {
            // Classify by first character: lowercase = rotation, uppercase = face
            if name.chars().next().map_or(false, |c| c.is_lowercase()) {
                rotation_moves.push(name.clone());
            } else {
                outer_moves.push(name.clone());
            }
        }

        println!(
            "  Outer face moves ({}): {}",
            outer_moves.len(),
            outer_moves.join(" ")
        );
        println!(
            "  Rotations ({}): {}",
            rotation_moves.len(),
            rotation_moves.join(" ")
        );
        println!(
            "  Arena stride: {} u128 slots per cube",
            movetable.arena().stride()
        );
        println!();
    }
}
