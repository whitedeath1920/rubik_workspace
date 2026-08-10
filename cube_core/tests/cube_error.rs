//! Tests for `CubeError` — all variants, Display, Debug, and Error trait.

use cube_core::{
    CubeError,
    cube_moves::{Axis, Face, LayerSpec, Move, MoveKind},
};

// ── 1. Display for every variant ───────────────────────────────────

#[test]
fn test_error_display_invalid_dimension() {
    let e = CubeError::InvalidDimension { got: 1 };
    let s = e.to_string();
    assert!(s.contains("Invalid dimension"), "got: {s}");
    assert!(s.contains("1"), "got: {s}");
}

#[test]
fn test_error_display_invalid_orientation() {
    let e = CubeError::InvalidOrientation {
        got: vec![(0, 5)],
        mod_: 3,
    };
    let s = e.to_string();
    assert!(s.contains("Invalid orientation"), "got: {s}");
    assert!(s.contains("3"), "got: {s}");
}

#[test]
fn test_error_display_invalid_permutation() {
    let e = CubeError::InvalidPermutation {
        got: vec![(0, 5)],
    };
    let s = e.to_string();
    assert!(s.contains("Invalid permutation"), "got: {s}");
    assert!(s.contains("parity"), "got: {s}");
}

#[test]
fn test_error_display_invalid_move_m() {
    let mv = Move {
        kind: MoveKind::FaceTurn {
            face: Face::U,
            layer: LayerSpec::Outer,
        },
        qturns: 1,
    };
    let e = CubeError::InvalidMoveM {
        got: mv,
        expected: vec![],
    };
    let s = e.to_string();
    assert!(s.contains("Invalid move"), "got: {s}");
}

#[test]
fn test_error_display_invalid_move_s() {
    let e = CubeError::InvalidMoveS {
        got: "X".into(),
        expected: vec!["U".into()],
    };
    let s = e.to_string();
    assert!(s.contains("Invalid move"), "got: {s}");
    assert!(s.contains("X"), "got: {s}");
}

#[test]
fn test_error_display_invalid_move_conjugate() {
    let mv = Move {
        kind: MoveKind::FaceTurn {
            face: Face::U,
            layer: LayerSpec::Outer,
        },
        qturns: 1,
    };
    let e = CubeError::InvalidMoveConjugate { got: mv };
    let s = e.to_string();
    assert!(s.contains("Invalid move"), "got: {s}");
}

#[test]
fn test_error_display_invalid_vector_dimension() {
    let e = CubeError::InvalidVectorDimension {
        i: 0,
        p: cube_core::Point3::new(0, 0, 0),
        half: 3,
    };
    let s = e.to_string();
    assert!(s.contains("Piece"), "got: {s}");
    assert!(s.contains("out of bounds"), "got: {s}");
}

#[test]
fn test_error_display_duplicate_vector() {
    let e = CubeError::DuplicateVector {
        i: 5,
        p: cube_core::Point3::new(1, 2, 3),
        hash: 42,
        other: 3,
        other_p: cube_core::Point3::new(1, 2, 3),
    };
    let s = e.to_string();
    assert!(s.contains("Duplicate position"), "got: {s}");
    assert!(s.contains("4"), "got: {s}"); // hash 42
}

#[test]
fn test_error_display_invalid_length() {
    let e = CubeError::InvalidLength {
        got: 10,
        expected: 26,
    };
    let s = e.to_string();
    assert!(s.contains("expected"), "got: {s}");
    assert!(s.contains("10"), "got: {s}");
    assert!(s.contains("26"), "got: {s}");
}

// ── 2. Debug formatting ────────────────────────────────────────────

#[test]
fn test_error_debug_all_variants() {
    // All variants must produce non-empty Debug output
    let errors: Vec<CubeError> = vec![
        CubeError::InvalidDimension { got: 0 },
        CubeError::InvalidOrientation {
            got: vec![],
            mod_: 2,
        },
        CubeError::InvalidPermutation {
            got: vec![],
        },
        CubeError::InvalidMoveM {
            got: Move {
                kind: MoveKind::FaceTurn {
                    face: Face::U,
                    layer: LayerSpec::Outer,
                },
                qturns: 1,
            },
            expected: vec![],
        },
        CubeError::InvalidMoveS {
            got: String::new(),
            expected: vec![],
        },
        CubeError::InvalidMoveConjugate {
            got: Move {
                kind: MoveKind::Rotation { axis: Axis::X },
                qturns: 1,
            },
        },
        CubeError::InvalidVectorDimension {
            i: 0,
            p: cube_core::Point3::new(0, 0, 0),
            half: 1,
        },
        CubeError::DuplicateVector {
            i: 0,
            p: cube_core::Point3::new(0, 0, 0),
            hash: 0,
            other: 1,
            other_p: cube_core::Point3::new(1, 1, 1),
        },
        CubeError::InvalidLength {
            got: 0,
            expected: 0,
        },
    ];
    for e in &errors {
        let debug = format!("{:?}", e);
        assert!(!debug.is_empty(), "empty Debug for {:?}", e);
    }
}

// ── 3. Error trait ─────────────────────────────────────────────────

#[test]
fn test_error_implements_std_error() {
    fn _assert_error<T: std::error::Error>(_: &T) {}
    let e = CubeError::InvalidDimension { got: 0 };
    _assert_error(&e);
}

#[test]
fn test_error_source_is_none() {
    use std::error::Error;
    let e = CubeError::InvalidPermutation {
        got: vec![(0, 5)],
    };
    assert!(e.source().is_none());
}

// ── 4. Result type alias ───────────────────────────────────────────

#[test]
fn test_result_type() {
    // cube_core::error::Result<T> is core::result::Result<T, CubeError>
    let ok: cube_core::error::Result<u32> = Ok(42);
    assert_eq!(ok.unwrap(), 42);

    let err: cube_core::error::Result<u32> = Err(CubeError::InvalidPermutation {
        got: vec![(0, 5)],
    });
    assert!(err.is_err());
}
