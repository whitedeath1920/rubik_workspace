//! # Packed Format Roundtrip
//!
//! Demonstrates the packed `u128` representation used for efficient
//! cube state storage. Shows how to:
//!   - Pack a CubeVect into a compact `Vec<u128>`
//!   - Store it in a CubeArena slot via `cube_from_slice`
//!   - Recover it via `cube_to_vec` and reconstruct a CubeVect
//!
//! This is the foundation for database storage, network transmission,
//! and pattern indexing — essential for large-scale search algorithms.
//!
//! Usage: `cargo run --example pack_roundtrip`

use cube_core::{
    CubeArena, CubeVect,
    cube_moves::{Face, LayerSpec, Move, MoveKind},
};

fn main() {
    let dimensions = [2usize, 3, 4, 5];

    for &n in &dimensions {
        println!("=== {n}×{n}×{n} Packed Roundtrip ===");

        // ── Step 1: Create a solved CubeVect ─────────────────────────
        let mut cube = CubeVect::new(n);
        println!(
            "  Created solved CubeVect with {} cubies",
            cube.data().len()
        );

        // ── Step 2: Apply a U move and check validity ────────────────
        let u_move = Move {
            kind: MoveKind::FaceTurn {
                face: Face::U,
                layer: LayerSpec::Outer,
            },
            qturns: 1,
        };
        cube.mv(u_move);
        assert!(cube.check().is_ok(), "U move produced invalid state");
        println!("  Applied U move — cube state is valid");

        // ── Step 3: Pack into compact representation ─────────────────
        let packed: Vec<u128> = cube.into_packed();
        println!("  Packed into {} × u128 values", packed.len());
        println!("  First u128:  {:#034x}", packed[0]);

        // ── Step 4: Store in a CubeArena slot ────────────────────────
        let mut arena = CubeArena::new_arena(n as u8, 2);
        arena.cube_from_slice(0, &packed);
        println!("  Stored in CubeArena slot 0");

        // ── Step 5: Recover via cube_to_vec / into_cubevect ──────────
        let recovered_as_vect = arena.cube_to_vec(0);
        println!(
            "  Recovered as vector of {} orbit groups",
            recovered_as_vect.len()
        );

        // ── Step 6: Reconstruct CubeVect from arena ──────────────────
        let reconstructed = arena.into_cubevect(0);
        assert!(
            reconstructed.check().is_ok(),
            "Reconstructed cube is invalid"
        );

        // ── Step 7: Verify the roundtrip ─────────────────────────────
        // Apply inverse move to reconstructed cube and verify it's solved
        let u_inv = u_move.invert();
        let solved = CubeVect::new(n);
        let mut test = reconstructed.clone();
        test.mv(u_inv);

        // Compare packed forms (more robust than CubeVect equality)
        let repacked = test.into_packed();
        let solution = solved.into_packed();
        if repacked == solution {
            println!("  ✓ Full roundtrip successful: CubeVect → packed → arena → CubeVect");
            println!("    Inverse move returned to solved state");
        } else {
            println!("  ✗ Roundtrip produced different result");
        }

        println!();
    }

    // ── Bonus: Show packed size efficiency ──────────────────────────
    println!("=== Storage Efficiency ===");
    for n in [2u8, 3, 5, 10, 20] {
        let arena = CubeArena::new_arena(n, 1);
        let stride = arena.stride();
        let bytes = stride as usize * 16; // each u128 = 16 bytes
        println!("  n={n}: {stride}×u128 = {bytes} bytes per cube state");
    }
}
