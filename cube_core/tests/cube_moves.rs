use cube_core::{
    CubeMoves, CubeState,
    cube_moves::{MoveClass, MoveRules, Turn},
};
#[test]
fn test_rotation() {
    let dimension = 7;
    let rule = MoveRules {
        moves: vec![MoveClass::Rotation],
        turns: vec![Turn::Clockwise, Turn::Anticlockwise, Turn::Double],
    };
    let moveset = CubeMoves::new(dimension, &rule);

    let l_moves = vec![
        "x".to_string(),
        "y".to_string(),
        "z".to_string(),
        "x'".to_string(),
        "y'".to_string(),
        "z'".to_string(),
        "x2".to_string(),
        "y2".to_string(),
        "z2".to_string(),
    ];

    let keys: Vec<&String> = moveset.moves_s.keys().collect();
    for mv in keys {
        if !l_moves.contains(&mv) {
            panic!("Move {} not found", mv);
        }
    }
}
#[test]
fn test_slice() {
    let dimension = 7;
    let rule = MoveRules {
        moves: vec![MoveClass::Slice],
        turns: vec![Turn::Clockwise, Turn::Anticlockwise, Turn::Double],
    };
    let moveset = CubeMoves::new(dimension, &rule);

    let l_moves = vec![
        "2U".to_string(),
        "3U".to_string(),
        "2F".to_string(),
        "3F".to_string(),
        "2R".to_string(),
        "3R".to_string(),
        "2D".to_string(),
        "3D".to_string(),
        "2B".to_string(),
        "3B".to_string(),
        "2L".to_string(),
        "3L".to_string(),
        "2U'".to_string(),
        "3U'".to_string(),
        "2F'".to_string(),
        "3F'".to_string(),
        "2R'".to_string(),
        "3R'".to_string(),
        "2D'".to_string(),
        "3D'".to_string(),
        "2B'".to_string(),
        "3B'".to_string(),
        "2L'".to_string(),
        "3L'".to_string(),
        "2U2".to_string(),
        "3U2".to_string(),
        "2F2".to_string(),
        "3F2".to_string(),
        "2R2".to_string(),
        "3R2".to_string(),
        "2D2".to_string(),
        "3D2".to_string(),
        "2B2".to_string(),
        "3B2".to_string(),
        "2L2".to_string(),
        "3L2".to_string(),
    ];

    let keys: Vec<&String> = moveset.moves_s.keys().collect();
    for mv in keys {
        if !l_moves.contains(&mv) {
            panic!("Move {} not found", mv);
        }
    }
}
#[test]
fn test_wide() {
    let dimension = 7;
    let rule = MoveRules {
        moves: vec![MoveClass::Wide],
        turns: vec![Turn::Clockwise, Turn::Anticlockwise, Turn::Double],
    };
    let moveset = CubeMoves::new(dimension, &rule);

    let l_moves = vec![
        "Uw".to_string(),
        "Uw'".to_string(),
        "Uw2".to_string(),
        "Fw".to_string(),
        "Fw'".to_string(),
        "Fw2".to_string(),
        "Rw".to_string(),
        "Rw'".to_string(),
        "Rw2".to_string(),
        "Dw".to_string(),
        "Dw'".to_string(),
        "Dw2".to_string(),
        "Bw".to_string(),
        "Bw'".to_string(),
        "Bw2".to_string(),
        "Lw".to_string(),
        "Lw'".to_string(),
        "Lw2".to_string(),
        "3Uw".to_string(),
        "3Uw'".to_string(),
        "3Uw2".to_string(),
        "3Fw".to_string(),
        "3Fw'".to_string(),
        "3Fw2".to_string(),
        "3Rw".to_string(),
        "3Rw'".to_string(),
        "3Rw2".to_string(),
        "3Dw".to_string(),
        "3Dw'".to_string(),
        "3Dw2".to_string(),
        "3Bw".to_string(),
        "3Bw'".to_string(),
        "3Bw2".to_string(),
        "3Lw".to_string(),
        "3Lw'".to_string(),
        "3Lw2".to_string(),
    ];

    let keys: Vec<&String> = moveset.moves_s.keys().collect();
    for mv in keys {
        if !l_moves.contains(&mv) {
            panic!("Move {} not found", mv);
        }
    }
}
#[test]
fn test_outer() {
    let dimension = 7;
    let rule = MoveRules {
        moves: vec![MoveClass::Outer],
        turns: vec![Turn::Clockwise, Turn::Anticlockwise, Turn::Double],
    };
    let moveset = CubeMoves::new(dimension, &rule);

    let l_moves = vec![
        "U".to_string(),
        "F".to_string(),
        "R".to_string(),
        "D".to_string(),
        "B".to_string(),
        "L".to_string(),
        "U'".to_string(),
        "F'".to_string(),
        "R'".to_string(),
        "D'".to_string(),
        "B'".to_string(),
        "L'".to_string(),
        "U2".to_string(),
        "F2".to_string(),
        "R2".to_string(),
        "D2".to_string(),
        "B2".to_string(),
        "L2".to_string(),
    ];

    let keys: Vec<&String> = moveset.moves_s.keys().collect();
    for mv in keys {
        if !l_moves.contains(&mv) {
            panic!("Move {} not found", mv);
        }
    }
}
#[test]
fn test_move_s() {
    let dimension = 3;
    let rule = MoveRules {
        moves: vec![
            MoveClass::Outer,
            MoveClass::Rotation,
            MoveClass::Wide,
            MoveClass::Slice,
        ],
        turns: vec![Turn::Clockwise, Turn::Anticlockwise, Turn::Double],
    };
    let moveset = CubeMoves::new(dimension, &rule);
    let mut cube = CubeState::new(dimension);
    moveset.make_move_s("R", &mut cube);
    let perm = vec![
        vec![0, 1, 3, 7, 4, 5, 2, 6, 0],
        vec![0, 1, 2, 7, 4, 5, 3, 11, 8, 9, 10, 6, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori = vec![
        vec![0, 0, 1, 2, 0, 0, 2, 1, 0],
        vec![0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1],
    ];
    let cube1 = CubeState::from_vec((perm, ori));

    assert_eq!(cube, cube1);
}
