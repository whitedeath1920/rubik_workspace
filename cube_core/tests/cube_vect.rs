use cube_core::{
    moves::{Faces, Layers, Move, MoveKind, get_dim_from_len, CubeVect},
    state::CubeState
};

#[test]
fn test_new() {
    let dimension = 34;
    let cube: CubeState = CubeVect::new(dimension).into();
    let cube_perm = CubeState::new(dimension);

    assert_eq!(cube, cube_perm);
}

#[test]
fn test_get_dim() {
    assert_eq!(get_dim_from_len(11, false), 7);
    assert_eq!(get_dim_from_len(7, true), 6);
    assert_eq!(get_dim_from_len(6, false), 5);
    assert_eq!(get_dim_from_len(3, true), 4);
    assert_eq!(get_dim_from_len(3, false), 3);
    assert_eq!(get_dim_from_len(1, true), 2);
}

#[test]
fn test_move() {
    let dimension = 3;
    let cube = CubeVect::new(dimension);
    let mv = Move {
        kind: MoveKind::FaceTurn {
            face: Faces::R,
            layers: Layers::Outer,
        },
        qturns: 1,
    };

    let cube_r = cube.mv(mv);
    let cube_r: CubeState = cube_r.into();
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

    assert_eq!(cube_r, cube1);
}
