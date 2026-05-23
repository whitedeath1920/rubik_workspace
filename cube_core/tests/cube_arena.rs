use cube_core::arena::CubeArena;

#[test]
fn new_slice() {
    let dimension = 7;
    let mut arena = CubeArena::new_arena(dimension, 2);
    let slc = [
        247132686368 | (407901468851537952 << 40) | (172066848 << 100),
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
        984818244535754528103549039458486304,
    ];
    arena.cube_from_slice(1, &slc);
    assert_eq!(arena.get_cube(1), arena.get_cube(0));
    assert_eq!(arena.get_cube(0), slc);
}
#[test]
fn add() {
    let dimension = 3;
    let mut arena = CubeArena::new_arena(dimension, 6);
    let pos1 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    arena.cube_from_vec(1, &pos1);

    let pos2 = vec![
        vec![(0, 0), (1, 0), (3, 1), (7, 2), (4, 0), (5, 0), (2, 2), (6, 1)],
        vec![(0, 0), (1, 0), (7, 0), (3, 0), (4, 0), (5, 0), (2, 0), (10, 0), (8, 0), (9, 0), (6, 0), (11, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    arena.cube_from_vec(2, &pos2);

    let pos3 = vec![
        vec![(0, 0), (1, 0), (3, 1), (4, 2), (5, 0), (6, 0), (2, 2), (7, 1)],
        vec![(0, 0), (1, 0), (7, 0), (3, 0), (4, 0), (5, 0), (2, 0), (11, 0), (9, 0), (10, 0), (6, 0), (8, 0)],
        vec![(0, 0),(3, 0), (4, 0), (1, 0), (2,0), (5, 0)],
    ];
    arena.cube_from_vec(3, &pos3);
    // Identity
    arena.add(0,1,5);
    assert_eq!(arena.get_cube(5), arena.get_cube(1));
    arena.add(0, 4, 0);
    assert_eq!(arena.get_cube(0), arena.get_cube(4));
    arena.add(1, 0, 5);
    assert_eq!(arena.get_cube(5), arena.get_cube(1));
   
    // move cycle 4
    arena.add(1,1,5);
    arena.add(5,1,5);
    arena.add(5,1,5);
    assert_eq!(arena.get_cube(5), arena.get_cube(0));
    arena.add(2,2,5);
    arena.add(5,2,5);
    arena.add(5,2,5);
    assert_eq!(arena.get_cube(5), arena.get_cube(0));

    // move1 + move2 = move3
    arena.add(1, 2, 5);
    assert_eq!(arena.get_cube(5), arena.get_cube(3));
}
#[test]
fn sub() {
    let dimension = 3;
    let mut arena = CubeArena::new_arena(dimension, 6);

    let pos1 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    arena.cube_from_vec(1, &pos1);


    let pos2 = vec![
        vec![(0, 0), (1, 0), (3, 1), (7, 2), (4, 0), (5, 0), (2, 2), (6, 1)],
        vec![(0, 0), (1, 0), (7, 0), (3, 0), (4, 0), (5, 0), (2, 0), (10, 0), (8, 0), (9, 0), (6, 0), (11, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    arena.cube_from_vec(2, &pos2);


    let pos3 = vec![
        vec![(0, 0), (1, 0), (3, 1), (4, 2), (5, 0), (6, 0), (2, 2), (7, 1)],
        vec![(0, 0), (1, 0), (7, 0), (3, 0), (4, 0), (5, 0), (2, 0), (11, 0), (9, 0), (10, 0), (6, 0), (8, 0)],
        vec![(0, 0),(3, 0), (4, 0),(1,0),(2,0), (5, 0)],
    ];
    arena.cube_from_vec(3, &pos3);
    // Identity
    arena.sub(1,0,5);
    assert_eq!(arena.get_cube(5), arena.get_cube(1));
    arena.sub(1, 1, 5);
    assert_eq!(arena.get_cube(0), arena.get_cube(5));
    arena.sub(0, 0, 5);
    assert_eq!(arena.get_cube(5), arena.get_cube(0));
   
    // move cycle 4
    arena.sub(0,1,5);
    arena.sub(5,1,5);
    arena.sub(5,1,5);
    arena.sub(5,1,5);
    assert_eq!(arena.get_cube(5), arena.get_cube(0));
    arena.sub(0,2,5);
    arena.sub(5,2,5);
    arena.sub(5,2,5);
    arena.sub(5,2,5);
    assert_eq!(arena.get_cube(5), arena.get_cube(0));

    // move3 - move2  = move1
    arena.sub(3, 2, 5);
    arena.print_cube(5);
    arena.print_cube(1);
    assert_eq!(arena.get_cube(5), arena.get_cube(1));
   
}
#[test]
fn inverse() {
    let dimension = 3;
    let mut arena = CubeArena::new_arena(dimension, 6);

    let pos1 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    arena.cube_from_vec(1, &pos1);
    arena.neg(1, 2);
    arena.add(2, 1, 3);
    // Identity
    assert_eq!(arena.get_cube(3), arena.get_cube(0));
}
#[test]
fn mul() {
    let dimension = 3;
    let mut arena = CubeArena::new_arena(dimension, 3);

    let pos1 = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4,0)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)],
        vec![(0, 0), (2, 0), (3, 0), (4, 0), (1, 0), (5, 0)],
    ];
    arena.cube_from_vec(1, &pos1);

    // Identity
    arena.mul(1, 0, 2);
    assert_eq!(arena.get_cube(2), arena.get_cube(0));
    arena.mul(1, 1, 2);
    assert_eq!(arena.get_cube(2), arena.get_cube(1));
    // move cycle 4
    arena.mul(1, 4, 2);
    assert_eq!(arena.get_cube(2), arena.get_cube(0));

    // -1 * a = -a
    arena.neg(1, 2);
    arena.mul(1, -1, 1);
    assert_eq!(arena.get_cube(2), arena.get_cube(1));
    
}
