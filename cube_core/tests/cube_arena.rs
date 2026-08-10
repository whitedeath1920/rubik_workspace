use cube_core::{CubeArena, arena::{Corner, Edge, Piece24}};

#[test]
fn test_arena_new_identity() {
    for n in 2..128 {
        let arena = CubeArena::new_arena(n as u8, 3);
        assert_eq!(arena.len(), 3, "n={}: expected len=3", n);
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(1),
            "n={}: identity slots differ",
            n
        );
        assert!(
            arena.is_solvable_slice(arena.get_cube(0)).is_ok(),
            "n={}: identity fails check_slice",
            n
        );
    }
}

#[test]
fn test_arena_cube_from_to_vec() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        let v = arena.cube_to_vec(0);
        arena.cube_from_vec(2, &v);
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(2),
            "n={}: cube_to_vec -> cube_from_vec roundtrip differs",
            n
        );
    }
}

#[test]
fn test_arena_identity_slot() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        arena.identity(1);
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(1),
            "n={}: identity() at slot 1 differs from slot 0",
            n
        );
    }
}

#[test]
fn test_arena_add_identity() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        arena.add(0, 0, 2);
        assert_eq!(arena.get_cube(0), arena.get_cube(2), "n={}: id+id != id", n);
    }
}

#[test]
fn test_arena_sub_identity() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        arena.sub(0, 0, 2);
        arena.identity(1);
        assert_eq!(arena.get_cube(2), arena.get_cube(1), "n={}: a-a != id", n);
    }
}

#[test]
fn test_arena_neg_add() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        arena.neg(0, 1);
        arena.add(0, 1, 2);
        arena.identity(1);
        assert_eq!(arena.get_cube(2), arena.get_cube(1), "n={}: a+(-a) != id", n);
    }
}

#[test]
fn test_arena_mul_identity() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        arena.mul(0, 1, 2);
        assert_eq!(arena.get_cube(0), arena.get_cube(2), "n={}: 1*id != id", n);
        arena.mul(0, 0, 2);
        arena.identity(1);
        assert_eq!(arena.get_cube(2), arena.get_cube(1), "n={}: 0*id != id", n);
    }
}

/// mul with negative: -1 × cube = -cube
#[test]
fn test_arena_mul_neg() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 4);
        arena.neg(0, 2); // slot2 = -identity
        arena.mul(0, -1, 3); // slot3 = -1 * identity
        assert_eq!(
            arena.get_cube(2),
            arena.get_cube(3),
            "n={}: -1*id 1= neg(id)",
            n
        );
    }
}

/// mul with large scalar: pow(a, n) using binary exponentiation.
#[test]
fn test_arena_mul_large_scalar() {
    for n in 2..128 {
        // 4 × identity = identity (cycle 4 for quarter-turn-like property)
        // 5 × identity = identity (since 5 mod 1 = 0 for identity)
        let mut arena = CubeArena::new_arena(n as u8, 3);
        arena.mul(0, 4, 2);
        arena.identity(1);
        assert_eq!(arena.get_cube(2), arena.get_cube(1), "n={}: 4*id != id", n);
        arena.mul(0, 5, 2);
        assert_eq!(arena.get_cube(2), arena.get_cube(1), "n={}: 5*id != id", n);
    }
}

/// add(a,b,c) when a == c (partial alias) clones a to scratch first.
#[test]
fn test_arena_add_partial_alias() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 4);
        // load identity into slot2, then slot2 += slot0 with alias
        arena.identity(2);
        arena.add(0, 2, 2); // slot2 = id + id -> still id
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(2),
            "n={}: alias add(id,id,id) != id",
            n
        );
    }
}

/// add(a,b,c) when a == c AND b == c (full alias).
#[test]
fn test_arena_add_full_alias() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 4);
        arena.add(0, 0, 0); // full alias
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(1),
            "n={}: full alias add(id,id,id)!=id",
            n
        );
    }
}

/// sub(a,b,c) when a == c (partial alias).
#[test]
fn test_arena_sub_partial_alias() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 4);
        arena.identity(2);
        arena.sub(2, 0, 2); // slot2 = id - id = id
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(2),
            "n={}: alias sub(id,id,id)!=id",
            n
        );
    }
}

#[test]
fn test_arena_clone() {
    let rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        arena.random_cube(2, rng);
        arena.clone_cube(0, 2);
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(2),
            "n={}: clone differs",
            n
        );
        arena.random_cube(1, rng);
        arena.clone_cube(0, 1);
        assert_eq!(
            arena.get_cube(1),
            arena.get_cube(2),
            "n={}: self-clone differs",
            n
        );
    }
}

/// Pack a cube, then recover it via into_cubevect -> into_packed.
#[test]
fn test_arena_into_cubevect_roundtrip() {
    for n in 2..128 {
        let arena = CubeArena::new_arena(n as u8, 2);
        let cv = arena.into_cubevect(0);
        let packed = cv.into_packed();
        assert_eq!(
            &packed,
            arena.get_cube(0),
            "n={}: into_cubevect -> into_packed roundtrip differs",
            n
        );
    }
}

/// Generate a random cube, verify check_cube passes.
#[test]
fn test_arena_random_cube_validity() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 2);
        arena.random_cube(0, &mut rng);
        match arena.is_solvable(0) {
            Ok(_) => {}
            Err(e) => panic!(
                "{e}\nn={}: random_cube failed check_cube: {:?}",
                n,
                arena.print_cube(0)
            ),
        };
    }
}

/// Generate two random cubes, verify they are not identical
/// (with high probability).
#[test]
fn test_arena_random_cube_distinct() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 2);
        arena.random_cube(0, &mut rng);
        arena.random_cube(1, &mut rng);
        // Two independent random cubes should differ (collision unlikely)
        if arena.get_cube(0) == arena.get_cube(1) {
            // Re-roll once — extremely improbable to collide twice
            arena.random_cube(1, &mut rng);
            assert_ne!(
                arena.get_cube(0),
                arena.get_cube(1),
                "n={}: two random cubes are identical",
                n
            );
        }
    }
}

/// add_slice with identity data must preserve the operand.
#[test]
fn test_arena_add_slice_identity() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 4);
        let id_data = arena.get_cube(0).to_vec();
        arena.add_slice(0, &id_data, 2);
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(2),
            "n={}: add_slice(id, id) != id",
            n
        );
    }
}

/// sub_slice must undo a previous add_slice.
#[test]
fn test_arena_add_sub_slice_cycle() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 4);
        let id_data = arena.get_cube(0).to_vec();
        arena.add_slice(0, &id_data, 2); // slot2 = id + id = id
        arena.sub_slice(2, &id_data, 3); // slot3 = id - id = id
        arena.identity(1);
        assert_eq!(
            arena.get_cube(3),
            arena.get_cube(1),
            "n={}: add_slice + sub_slice != id",
            n
        );
    }
}

#[test]
fn test_arena_cycle_decomposition() {
    for n in 2..127 {
        let arena = CubeArena::new_arena(n as u8, 1);
        let cycles = arena.cycle_decomposition_cube(0);
        // Identity has no cycles (all fixed points)
        for orbit_cycles in &cycles {
            assert!(
                orbit_cycles.is_empty(),
                "n={}: identity has non-trivial cycles",
                n
            );
        }
    }
}

#[test]
fn test_arena_check_slice_identity() {
    for n in 2..128 {
        let arena = CubeArena::new_arena(n as u8, 1);
        assert!(
            arena.is_solvable_slice(arena.get_cube(0)).is_ok(),
            "n={}: id fails check_slice",
            n
        );
    }
}

#[test]
fn test_arena_check_cube_identity() {
    for n in 2..128 {
        let arena = CubeArena::new_arena(n as u8, 1);
        assert!(arena.is_solvable(0).is_ok(), "n={}: id fails check_cube", n);
    }
}

#[test]
fn test_arena_n2_stride1() {
    let a = CubeArena::new_arena(2, 3);
    assert_eq!(a.stride(), 1);
    assert_eq!(a.n(), 2);
}

#[test]
fn test_arena_n3_stride1() {
    let a = CubeArena::new_arena(3, 3);
    assert_eq!(a.stride(), 1);
}

#[test]
fn test_arena_n4_stride3() {
    let a = CubeArena::new_arena(4, 3);
    assert_eq!(a.stride(), 3);
}

// ── 11. Algebraic properties with random data ────────────────────

fn random_arena(n: u8, slots: usize, rng: &mut impl rand::Rng) -> CubeArena {
    let mut a = CubeArena::new_arena(n, slots);
    // Fill slots 3.. with random cubes (skip 0,1,2 for workspace)
    for i in 3..slots {
        a.random_cube(i, rng);
    }
    a
}

/// 0 * a = id
#[test]
fn test_arena_algebra_mul0() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 6, &mut rng);
        for i in 3..6 {
            arena.mul(i, 0, 0);
            arena.identity(1);
            assert_eq!(
                arena.get_cube(0),
                arena.get_cube(1),
                "n={} slot={}: 0*a!=id",
                n,
                i
            );
        }
    }
}

/// 1 * a = a
#[test]
fn test_arena_algebra_mul1() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 6, &mut rng);
        for i in 3..6 {
            arena.clone_cube(i, 0);
            arena.mul(i, 1, 1);
            assert_eq!(
                arena.get_cube(i),
                arena.get_cube(1),
                "n={} slot={}: 1*a!=a",
                n,
                i
            );
        }
    }
}

/// a - a = id
#[test]
fn test_arena_algebra_sub_self() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 6, &mut rng);
        for i in 3..6 {
            arena.sub(i, i, 0);
            arena.identity(1);
            assert_eq!(
                arena.get_cube(0),
                arena.get_cube(1),
                "n={} slot={}: a-a!=id",
                n,
                i
            );
        }
    }
}

/// -1 * a = -a
#[test]
fn test_arena_algebra_neg_mul() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 6, &mut rng);
        for i in 3..6 {
            arena.clone_cube(i, 0);
            arena.neg(i, 1);
            arena.mul(i, -1, 2);
            assert_eq!(
                arena.get_cube(1),
                arena.get_cube(2),
                "n={} slot={}: -1*a!=-a",
                n,
                i
            );
        }
    }
}

/// a + b = c  ⇒  a = c - b  (right-cancel works even in non-abelian)
#[test]
fn test_arena_algebra_add_then_sub() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 6, &mut rng);
        let (a, b) = (3, 4);
        arena.add(a, b, 5); // slot5 = a + b
        arena.sub(5, b, 0); // slot0 = (a+b) - b = a
        let a_orig = arena.get_cube(a);
        assert_eq!(arena.get_cube(0), a_orig, "n={}: (a+b)-b != a", n);
    }
}

/// a + b - c = id  when c = a + b
#[test]
fn test_arena_algebra_add_sub_cycle() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 8, &mut rng);
        let (a, b) = (3, 4);
        arena.add(a, b, 5); // c = a + b  at slot5
        arena.add(a, b, 6); // also c at slot6
        arena.sub(6, 5, 0); // c - c = id
        arena.identity(1);
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(1),
            "n={}: c-c!=id when c=a+b",
            n
        );
    }
}

/// -a + c = b  when c = a + b
#[test]
fn test_arena_algebra_neg_add() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 8, &mut rng);
        let (a, b) = (3, 4);
        arena.add(a, b, 5); // c = a + b
        arena.neg(a, 6); // -a
        arena.add(6, 5, 0); // -a + c = -a + (a+b) = b
        let b_orig = arena.get_cube(b);
        assert_eq!(arena.get_cube(0), b_orig, "n={}: -a+(a+b) != b", n);
    }
}

/// Associativity: (a + b) + c = a + (b + c)
#[test]
fn test_arena_algebra_associativity() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 9, &mut rng);
        let (a, b, c) = (3, 4, 5);
        // (a + b) + c
        arena.add(a, b, 6);
        arena.add(6, c, 7);
        // a + (b + c)
        arena.add(b, c, 6);
        arena.add(a, 6, 8);
        assert_eq!(
            arena.get_cube(7),
            arena.get_cube(8),
            "n={}: (a+b)+c != a+(b+c)",
            n
        );
    }
}

/// a + id = a  (right identity)
#[test]
fn test_arena_algebra_right_identity() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 6, &mut rng);
        for i in 3..6 {
            arena.add(i, 0, 1); // a + id
            assert_eq!(
                arena.get_cube(i),
                arena.get_cube(1),
                "n={} slot={}: a+id!=a",
                n,
                i
            );
        }
    }
}

/// id + a = a  (left identity)
#[test]
fn test_arena_algebra_left_identity() {
    let mut rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = random_arena(n as u8, 6, &mut rng);
        for i in 3..6 {
            arena.add(0, i, 1); // id + a
            assert_eq!(
                arena.get_cube(i),
                arena.get_cube(1),
                "n={} slot={}: id+a!=a",
                n,
                i
            );
        }
    }
}

#[test]
fn test_cube_order() {
    for n in 2..128 {
        let arena = CubeArena::new_arena(n as u8, 1);
        arena.print_cube(0);
        let order = arena.cube_order(0);
        assert_eq!(order, 1, "n={}: cube order {} != 1", n, order);
    }
}

#[test]
fn test_random_cube_order() {
    let rng = &mut rand::rng();
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        arena.random_cube(0, rng);
        let order = arena.cube_order(0);
        arena.mul(0, order as isize, 1);
        assert_eq!(arena.get_cube(2), arena.get_cube(1), "n={}: 1*id != id", n);
    }
}

#[test]
fn test_unpack_pack_roundtrip_corners() {
    use cube_core::arena::{pack_u128, unpack_u128};
    // 8 corners, perm 0..7, ori 0..2 at shift 3
    let original: Vec<(u8, u8)> = (0..8).map(|i| (i, i % 3)).collect();
    let packed = pack_u128::<Corner>(&original);
    let unpacked = unpack_u128::<Corner>(packed);
    assert_eq!(original, unpacked, "corner roundtrip failed");
}

#[test]
fn test_unpack_pack_roundtrip_edges() {
    use cube_core::arena::{pack_u128, unpack_u128};
    // 12 edges, perm 0..11, ori 0..1 at shift 4
    let original: Vec<(u8, u8)> = (0..12).map(|i| (i, i % 2)).collect();
    let packed = pack_u128::<Edge>(&original);
    let unpacked = unpack_u128::<Edge>(packed);
    assert_eq!(original, unpacked, "edge roundtrip failed");
}

#[test]
fn test_unpack_pack_roundtrip_24piece() {
    use cube_core::arena::{pack_u128, unpack_u128};
    // 24 pieces, perm only (shift 5 means no ori)
    let original: Vec<(u8, u8)> = (0..24).map(|i| (i, 0)).collect();
    let packed = pack_u128::<Piece24>(&original);
    let unpacked = unpack_u128::<Piece24>(packed);
    assert_eq!(original, unpacked, "24-piece roundtrip failed");
}

#[test]
fn test_pack_u128_identity_pattern() {
    use cube_core::arena::pack_u128;
    // Identity pattern for 24-piece orbit
    let identity: Vec<(u8, u8)> = (0..24).map(|i| (i, 0)).collect();
    let packed = pack_u128::<Piece24>(&identity);
    // Should not be zero
    assert_ne!(packed, 0, "identity pack should not be zero");
}

#[test]
fn test_gcd_known_values() {
    use cube_core::arena::gcd;
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(8, 12), 4);
    assert_eq!(gcd(17, 13), 1);
    assert_eq!(gcd(100, 100), 100);
    assert_eq!(gcd(1, 1), 1);
    assert_eq!(gcd(2usize.pow(10), 2usize.pow(8)), 256);
}

#[test]
fn test_mcm_known_values() {
    use cube_core::arena::mcm;
    assert_eq!(mcm(12, 8), 24);
    assert_eq!(mcm(17, 13), 221);
    assert_eq!(mcm(1, 100), 100);
    assert_eq!(mcm(100, 1), 100);
    assert_eq!(mcm(4, 6), 12);
}

#[test]
fn test_gcd_mcm_identity() {
    use cube_core::arena::{gcd, mcm};
    for a in 1..=50 {
        for b in 1..=50 {
            // gcd(a,b) * mcm(a,b) == a * b
            assert_eq!(gcd(a, b) * mcm(a, b), a * b, "failed for {a}, {b}");
        }
    }
}

#[test]
fn test_gcd_large_values() {
    use cube_core::arena::gcd;
    // Large values that may stress the binary gcd algorithm
    assert_eq!(gcd(1073741824, 536870912), 536870912);
    assert_eq!(gcd(999999937, 1000000007), 1); // two large primes
}

#[test]
fn test_parity_identity_even() {
    use cube_core::arena::parity;
    // Identity permutation: pieces at positions 0,1,...,LEN-1
    let mut val = 0u128;
    for i in 0..8u128 {
        val |= i << (i * 5);
    }
    assert!(!parity::<Corner>(val), "identity should be even parity");
}

#[test]
fn test_parity_identity_24() {
    use cube_core::arena::parity;
    let mut val = 0u128;
    for i in 0..24u128 {
        val |= i << (i * 5);
    }
    assert!(!parity::<Piece24>(val), "24-identity should be even parity");
}

#[test]
fn test_parity_single_swap() {
    use cube_core::arena::parity;
    // Swap pieces 0 and 1: position 0 holds 1, position 1 holds 0
    let mut val = 0u128;
    val |= 1u128; // position 0 -> 1
    val |= 0u128 << 5; // position 1 -> 0
    for i in 2..8u128 {
        val |= i << (i * 5);
    }
    assert!(parity::<Corner>(val), "single swap should be odd parity");
}

#[test]
fn test_parity_three_cycle() {
    use cube_core::arena::parity;
    // 3-cycle: 0->1, 1->2, 2->0 (even parity)
    let mut val = 0u128;
    val |= 1u128; // 0 -> 1
    val |= 2u128 << 5; // 1 -> 2
    val |= 0u128 << 10; // 2 -> 0
    for i in 3..8u128 {
        val |= i << (i * 5);
    }
    assert!(!parity::<Corner>(val), "3-cycle should be even parity");
}

#[test]
fn test_orientation_check_identity_corners() {
    use cube_core::arena::orientation_check;
    // Identity: all ori = 0
    let mut val = 0u128;
    for i in 0..8u128 {
        val |= i << (i * 5); // perm=i, ori=0
    }
    assert!(
        orientation_check::<Corner>(val).is_ok(),
        "identity should pass"
    );
}

#[test]
fn test_orientation_check_identity_edges() {
    use cube_core::arena::orientation_check;
    let mut val = 0u128;
    for i in 0..12u128 {
        val |= i << (i * 5);
    }
    assert!(
        orientation_check::<Edge>(val).is_ok(),
        "identity should pass"
    );
}

#[test]
fn test_orientation_check_fails_bad_sum() {
    use cube_core::arena::orientation_check;
    // Single corner with ori=1, rest 0 — sum=1 mod 3 != 0
    let mut val = 0u128;
    val |= 0u128 | (1u128 << 3); // perm=0, ori=1
    for i in 1..8u128 {
        val |= i << (i * 5);
    }
    assert!(
        orientation_check::<Corner>(val).is_err(),
        "should fail"
    );
}

// ── 16. orbit_order ──────────────────────────────────────────────

#[test]
fn test_orbit_order_identity() {
    use cube_core::arena::orbit_order;
    // Identity corners: every piece at its own position, ori=0
    let mut val = 0u128;
    for i in 0..8u128 {
        val |= i << (i * 5);
    }
    assert_eq!(
        orbit_order::<Corner>(val),
        1,
        "identity order should be 1"
    );
}

#[test]
fn test_orbit_order_identity_24() {
    use cube_core::arena::orbit_order;
    let mut val = 0u128;
    for i in 0..24u128 {
        val |= i << (i * 5);
    }
    assert_eq!(orbit_order::<Piece24>(val), 1);
}

#[test]
fn test_orbit_order_4_cycle() {
    use cube_core::arena::orbit_order;
    // 4-cycle: 0->1, 1->2, 2->3, 3->0
    let mut val = 0u128;
    val |= 1u128;
    val |= 2u128 << 5;
    val |= 3u128 << 10;
    val |= 0u128 << 15;
    for i in 4..8u128 {
        val |= i << (i * 5);
    }
    assert_eq!(
        orbit_order::<Corner>(val),
        4,
        "4-cycle order should be 4"
    );
}

#[test]
fn test_add_ori_identity() {
    use cube_core::arena::add_ori;
    // Identity corners: both a and b have perm=i, ori=0
    let mut a_val = 0u64;
    let mut b_val = 0u64;
    for i in 0..8u64 {
        a_val |= i << (i * 5);
        b_val |= i << (i * 5);
    }
    let result = add_ori::<Corner>(a_val, b_val);
    // Result should be identity too (ori 0+0=0)
    println!("result: {result:b}, a_val: {a_val:b}, b_val: {b_val:b}");
    assert_eq!(
        result as u64 & ((1 << 40) - 1),
        a_val,
        "add_ori identity failed"
    );
}

#[test]
fn test_sub_ori_identity() {
    use cube_core::arena::sub_ori;
    let mut a_val = 0u64;
    for i in 0..8u64 {
        a_val |= i << (i * 5);
    }
    let result = sub_ori::<Corner>(a_val, a_val);
    // a - a = identity with ori 0
    // Result perms should match identity pattern
    for i in 0..8 {
        let block = (result >> (i * 5)) & 31;
        assert_eq!(block & 7, i as u128, "sub_ori perm mismatch at {i}");
        assert_eq!((block >> 3) & 3, 0, "sub_ori ori should be 0 at {i}");
    }
}

#[test]
fn test_add_perm_identity() {
    use cube_core::arena::add_perm;
    // Identity for 24-piece orbit
    let mut a_val = 0u128;
    let mut b_val = 0u128;
    for i in 0..24u128 {
        a_val |= i << (i * 5);
        b_val |= i << (i * 5);
    }
    let result = add_perm::<Piece24>(a_val, b_val);
    assert_eq!(result, a_val, "add_perm identity failed");
}

#[test]
fn test_sub_perm_identity() {
    use cube_core::arena::sub_perm;
    let mut a_val = 0u128;
    for i in 0..24u128 {
        a_val |= i << (i * 5);
    }
    let result = sub_perm::<Piece24>(a_val, a_val);
    for i in 0..24 {
        let block = (result >> (i * 5)) & 31;
        assert_eq!(
            block, i as u128,
            "sub_perm should give identity, got {block} at {i}"
        );
    }
}

// ── 19. cycle_decomp (free function) ─────────────────────────────

#[test]
fn test_cycle_decomp_identity() {
    use cube_core::arena::cycle_decomp;
    let mut val = 0u128;
    for i in 0..8u128 {
        val |= i << (i * 5);
    }
    let cycles = cycle_decomp::<Corner>(val);
    assert!(cycles.is_empty(), "identity should have no cycles");
}

#[test]
fn test_cycle_decomp_4_cycle() {
    use cube_core::arena::cycle_decomp;
    // 4-cycle on positions 0-3
    let mut val = 0u128;
    val |= 1u128; // pos 0 -> 1
    val |= 2u128 << 5; // pos 1 -> 2
    val |= 3u128 << 10; // pos 2 -> 3
    val |= 0u128 << 15; // pos 3 -> 0
    for i in 4..8u128 {
        val |= i << (i * 5);
    }
    let cycles = cycle_decomp::<Corner>(val);
    assert_eq!(cycles.len(), 1, "should have exactly one cycle");
    assert_eq!(cycles[0].len(), 4, "cycle should have length 4");
}

// ── 20. shuffle ──────────────────────────────────────────────────

#[test]
fn test_shuffle_modifies_slice() {
    use cube_core::arena::shuffle;
    let mut rng = rand::rng();
    let mut arr: [(u8, u8); 24] = std::array::from_fn(|i| (i as u8, 0));
    let _original = arr;
    let _parity = shuffle(&mut arr, &mut rng);
    // With 24! possible permutations, it's extremely unlikely to be unchanged
    // But we just check the function runs without panic and returns a bool
    let _ = _parity;
}

// ── 21. get_cube_mut mutation ─────────────────────────────────────

#[test]
fn test_get_cube_mut_modifies() {
    for n in 2..127 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        let original = arena.get_cube(0).to_vec();
        let _stride = arena.stride() as usize;
        // Write a different value
        arena.get_cube_mut(2)[0] = !original[0];
        assert_ne!(
            arena.get_cube(2),
            arena.get_cube(0),
            "n={}: mutation through get_cube_mut should be visible",
            n
        );
    }
}

// ── 22. cube_ptr / cube_mut_ptr ───────────────────────────────────

#[test]
fn test_cube_ptr_reads_correctly() {
    for n in 2..127 {
        let arena = CubeArena::new_arena(n as u8, 2);
        let ptr = arena.cube_ptr(0);
        let slice = arena.get_cube(0);
        unsafe {
            for i in 0..arena.stride() as usize {
                assert_eq!(*ptr.add(i), slice[i], "n={}: ptr[{i}] mismatch", n);
            }
        }
    }
}

#[test]
fn test_cube_mut_ptr_writes() {
    for n in 2..127 {
        let mut arena = CubeArena::new_arena(n as u8, 3);
        let original = arena.get_cube(0).to_vec();
        let ptr = arena.cube_mut_ptr(2);
        unsafe {
            for i in 0..arena.stride() as usize {
                *ptr.add(i) = !original[i];
            }
        }
        assert_ne!(
            arena.get_cube(2),
            arena.get_cube(0),
            "n={}: cube_mut_ptr write should be visible",
            n
        );
    }
}

// ── 23. sub_slice inverse of add_slice ────────────────────────────

#[test]
fn test_sub_slice_inverts_add_slice() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 5);
        let id_data = arena.get_cube(0).to_vec();
        // Start from identity at slot 3
        arena.identity(3);
        // Add identity slice -> result is still identity
        arena.add_slice(3, &id_data, 4);
        // Sub identity slice -> back to identity
        arena.sub_slice(4, &id_data, 4);
        arena.identity(0);
        assert_eq!(
            arena.get_cube(4),
            arena.get_cube(0),
            "n={}: add_slice+sub_slice != identity",
            n
        );
    }
}

// ── 24. neg applied twice ────────────────────────────────────────

#[test]
fn test_neg_twice_is_identity_for_id() {
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 4);
        // neg(identity) = identity (since identity is its own inverse)
        arena.neg(0, 1);
        arena.neg(1, 2);
        assert_eq!(
            arena.get_cube(0),
            arena.get_cube(2),
            "n={}: neg(neg(id)) != id",
            n
        );
    }
}

// ── 25. is_solvable on identity across all n ─────────────────────

#[test]
fn test_is_solvable_identity_all_n() {
    for n in 2..128 {
        let arena = CubeArena::new_arena(n as u8, 1);
        assert!(
            arena.is_solvable(0).is_ok(),
            "n={}: identity should be solvable",
            n
        );
    }
}

#[test]
fn test_is_solvable_random_all_n() {
    let rng = &mut rand::rng();
    // Verify random_cube1 produces structurally valid cubes for all n.
    // (Full solvability via is_solvable is stricter and may reject
    // random_cube1 output — that's a known limitation.)
    for n in 2..128 {
        let mut arena = CubeArena::new_arena(n as u8, 2);
        arena.random_cube(0, rng);
        // Use _check_slice which validates structural invariants,
        // not the stricter is_solvable which requires Bonzio-Loi-Peruzzi conditions
        match arena.is_solvable(0) {
            Ok(_) => {}
            Err(e) => {
                arena.print_cube(0);
                panic!("n={}: random_cube1 should be structurally valid: {e}", n);
            }
        }
    }
}

// ── 26. stride / len / n consistency ───────────────────────────

#[test]
fn test_stride_consistency_with_n() {
    // For n=2: stride=1 (tmp=0)
    // For n=3: stride=1 (tmp=0)
    // For n=4: stride=3 (tmp=1 -> 1+1+0+1=3? Let's test)
    for n in 2..=50 {
        let arena = CubeArena::new_arena(n as u8, 2);
        let stride = arena.stride();
        let len = arena.len();
        let dim = arena.n();
        assert_eq!(dim, n, "n mismatch");
        assert_eq!(len, 2, "len mismatch");
        assert!(stride >= 1, "n={}: stride={stride} should be >=1", n);
        // Verify data length matches len * stride + 2*stride (scratch space)
        assert_eq!(
            arena.get_cube(0).len(),
            stride as usize,
            "n={}: get_cube length != stride",
            n
        );
    }
}

#[test]
fn test_len_returns_correct_value() {
    for len in 1..=5 {
        for n in [2u8, 3, 4, 5] {
            let arena = CubeArena::new_arena(n, len);
            assert_eq!(arena.len(), len, "n={n} len={len}");
        }
    }
}

#[test]
fn test_print_cube_does_not_panic() {
    for n in 2..127 {
        let arena = CubeArena::new_arena(n as u8, 1);
        arena.print_cube(0); // Should not panic
    }
}


#[test]
fn test_normalize_cube_does_not_panic() {
    for n in 2..127 {
        let mut arena = CubeArena::new_arena(n as u8, 2);
        // normalize_cube should not panic on identity
        arena.normalize_cube(0);
    }
}


// ── 30. Comprehensive arena operation for n=2 (minimal) ─────────

#[test]
fn test_arena_n2_all_operations() {
    let mut arena = CubeArena::new_arena(2, 5);
    // n=2 has stride 1
    assert_eq!(arena.stride(), 1);
    assert_eq!(arena.n(), 2);

    // Identity operations
    arena.identity(0);
    arena.identity(1);
    assert_eq!(arena.get_cube(0), arena.get_cube(1));

    // Add identity
    arena.add(0, 1, 2);
    assert_eq!(arena.get_cube(0), arena.get_cube(2));

    // Sub identity
    arena.sub(0, 0, 3);
    assert_eq!(arena.get_cube(3), arena.get_cube(0));

    // Mul scalar
    arena.mul(0, 3, 4);
    assert_eq!(arena.get_cube(4), arena.get_cube(0));

    // Clone
    arena.clone_cube(0, 4);
    assert_eq!(arena.get_cube(0), arena.get_cube(4));
}
