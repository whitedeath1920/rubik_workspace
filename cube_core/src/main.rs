use cube_core::{
    cube_moves::{MoveClass, MoveRules, Turn}, cube_perm::cube_perm::_to_vec, CubeMoves, CubePerm
};

fn main() {
    let dimension = 2;
    let rule = MoveRules {
        moves: vec![MoveClass::Outer, MoveClass::Rotation, MoveClass::Wide, MoveClass::Slice ],
        turns: vec![Turn::Clockwise, Turn::Double, Turn::Anticlockwise],
    };
    let moveset = CubeMoves::new(dimension, &rule);
    let mut cube = CubePerm::new(dimension);
    
    println!("cube:\nori: {:?} \t perm: {:?}\n", _to_vec(cube.ori[0]),_to_vec(cube.perm[0]));
    let alg = "R F2 R F R U F2 R U";
    for mv in alg.split(' ') {
        moveset.make_move_s(mv, &mut cube);
    }
    
    println!("alg: \t {}",alg);
    println!("cube:\nori: {:?} \t perm: {:?}\n", _to_vec(cube.ori[0]),_to_vec(cube.perm[0]));
}
