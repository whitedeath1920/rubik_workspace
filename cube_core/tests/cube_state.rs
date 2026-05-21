use cube_core::state::{CubeState, Layout};

#[test]
fn new_slice() {
    let dimension = 7;
    let layout = Layout::new(dimension);
    let cube = CubeState::with_layout(&layout);

    let slc = [
        247132686368,
        407901468851537952,
        172066848,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
    ];

    assert_eq!(CubeState::from_slice(&slc, &layout), cube);
    assert_eq!(cube.to_slice(), slc);
}
#[test]
fn new_vec() {
    let dimension = 7;
    let layout = Layout::new(dimension);
    let cube = CubeState::with_layout(&layout);

    let vect = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0), (21, 0), (22, 0), (23, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0), (21, 0), (22, 0), (23, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0), (21, 0), (22, 0), (23, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0), (21, 0), (22, 0), (23, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0), (21, 0), (22, 0), (23, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0), (21, 0), (22, 0), (23, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0), (21, 0), (22, 0), (23, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0), (21, 0), (22, 0), (23, 0)]
    ];

    assert_eq!(cube.to_vec(), vect);
    assert_eq!(CubeState::from_vec(vect, &layout), cube);
}
#[test]
fn add() {
    let dimension = 3;
    let layout = Layout::new(dimension);
    let solved = CubeState::with_layout(&layout);

    let pos1 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    let cube1 = CubeState::from_vec(pos1, &layout);


    let pos2 = vec![
        vec![(0, 0), (1, 0), (3, 1), (7, 2), (4, 0), (5, 0), (2, 2), (6, 1)],
        vec![(0, 0), (1, 0), (7, 0), (3, 0), (4, 0), (5, 0), (2, 0), (10, 0), (8, 0), (9, 0), (6, 0), (11, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    let cube2 = CubeState::from_vec(pos2, &layout);

    let pos3 = vec![
        vec![(0, 0), (1, 0), (3, 1), (4, 2), (5, 0), (6, 0), (2, 2), (7, 1)],
        vec![(0, 0), (1, 0), (7, 0), (3, 0), (4, 0), (5, 0), (2, 0), (11, 0), (9, 0), (10, 0), (6, 0), (8, 0)],
        vec![(0, 0),(3, 0), (4, 0), (1, 0), (2,0), (5, 0)],
    ];
    let cube3 = CubeState::from_vec(pos3, &layout);
    // Identity
    assert_eq!(cube1.clone() + &solved, cube1);
    assert_eq!(solved.clone() + &solved, solved);
    assert_eq!(solved.clone() + &cube1, cube1);
   
    // move cycle 4
    assert_eq!(cube1.clone() + &cube1 + &cube1 + &cube1, solved);
    assert_eq!(cube2.clone() + &cube2 + &cube2 + &cube2, solved);

    // move1 + move2 = move3
    assert_eq!(cube1.clone() + &cube2, cube3);
}
#[test]
fn sub() {
    let dimension = 3;
    let layout = Layout::new(dimension);
    let solved = CubeState::with_layout(&layout);

    let pos1 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    let cube1 = CubeState::from_vec(pos1, &layout);


    let pos2 = vec![
        vec![(0, 0), (1, 0), (3, 1), (7, 2), (4, 0), (5, 0), (2, 2), (6, 1)],
        vec![(0, 0), (1, 0), (7, 0), (3, 0), (4, 0), (5, 0), (2, 0), (10, 0), (8, 0), (9, 0), (6, 0), (11, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    let cube2 = CubeState::from_vec(pos2, &layout);

    let pos3 = vec![
        vec![(0, 0), (1, 0), (3, 1), (4, 2), (5, 0), (6, 0), (2, 2), (7, 1)],
        vec![(0, 0), (1, 0), (7, 0), (3, 0), (4, 0), (5, 0), (2, 0), (11, 0), (9, 0), (10, 0), (6, 0), (8, 0)],
        vec![(0, 0),(3, 0), (4, 0),(1,0),(2,0), (5, 0)],
    ];
    let cube3 = CubeState::from_vec(pos3, &layout);
    // Identity
    assert_eq!(cube1.clone() - &solved, cube1);
    assert_eq!(cube1.clone() - &cube1, solved);
    assert_eq!(solved.clone() - &solved, solved);
   
    // move cycle 4
    assert_eq!(solved.clone() - &cube1 - &cube1 - &cube1 - &cube1, solved);
    assert_eq!(solved.clone() - &cube2 - &cube2 - &cube2 - &cube2, solved);

    // move3 - move2 = move1
    assert_eq!(cube3.clone() - &cube2, cube1);
}
#[test]
fn inverse() {
    let dimension = 3;
    let layout = Layout::new(dimension);
    let solved = CubeState::with_layout(&layout);

    let pos1 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    let cube1 = CubeState::from_vec(pos1, &layout);
    // Identity
    assert_eq!(-cube1.clone() + &cube1, solved);
}
#[test]
fn mul() {
    let dimension = 3;
    let layout = Layout::new(dimension);
    let solved = CubeState::with_layout(&layout);

    let pos1 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    let cube1 = CubeState::from_vec(pos1, &layout);

    // Identity
    assert_eq!(0 * cube1.clone(), solved);
    assert_eq!(1 * cube1.clone(), cube1);
    // move cycle 4
    assert_eq!(4 * cube1.clone(), solved);

    // -1 * a = -a
    assert_eq!(-1 * cube1.clone(), -cube1);
}
#[test]
fn check() {
    let dimension = 3;
    let layout = Layout::new(dimension);
    let solved = CubeState::with_layout(&layout);

    let pos1 = vec![
        vec![(0, 1), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    let cube1 = CubeState::from_vec(pos1, &layout);
    let pos2 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)],
        vec![(0, 0), (2, 0), (1, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
    ];
    let cube2 = CubeState::from_vec(pos2, &layout);
    // true
    assert!(solved.check().is_ok());

    // wrong corner orientation
    assert!(cube1.check().is_err());

    // Wrong edge permutation
    assert!(cube2.check().is_err());
}
#[test]
fn modulus() {
    let dimension = 3;
    let layout = Layout::new(dimension);
    let solved = CubeState::with_layout(&layout);

    let pos1 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    let cube1 = CubeState::from_vec(pos1, &layout);

    let pos2 = vec![
        vec![(0, 0), (1, 0), (3, 1), (4, 2), (5, 0), (6, 0), (2, 2), (7, 1)],
        vec![(0, 0), (1, 0), (7, 0), (3, 0), (4, 0), (5, 0), (2, 0), (11, 0), (9, 0), (10, 0), (6, 0), (8, 0)],
        vec![(0, 0),(3, 0), (4, 0), (1, 0), (2,0), (5, 0)],
    ];
    let cube2 = CubeState::from_vec(pos2, &layout);

    assert_eq!(cube1.get_modulus(), 4);
    assert_eq!(cube2.get_modulus() * cube2.clone(), solved);
}