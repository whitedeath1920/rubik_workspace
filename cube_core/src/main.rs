use cube_core::{
    cube_moves::{MoveClass, MoveRules, Turn}, CubeMoves, CubePerm,
    cube_perm::ops::Bit,
};

fn main() {
    let dimension = 2;
    let rule = MoveRules {
        moves: vec![MoveClass::Outer, MoveClass::Rotation, MoveClass::Wide, MoveClass::Slice ],
        turns: vec![Turn::Clockwise, Turn::Double, Turn::Anticlockwise],
    };
    let moveset = CubeMoves::new(dimension, &rule);
    let mut cube = CubePerm::new(dimension);
    
    println!("cube:\nori: {:?} \t perm: {:?}\n", cube.ori[0].to_vec(),cube.perm[0].to_vec());
    let alg = "R F2 R F R U F2 R U";
    for mv in alg.split(' ') {
        moveset.make_move_s(mv, &mut cube);
    }
    
    println!("alg: \t {}",alg);
    println!("cube:\nori: {:?} \t perm: {:?}\n", cube.ori[0].to_vec(),cube.perm[0].to_vec());
}
