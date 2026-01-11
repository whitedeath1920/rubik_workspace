use cube_core::{
    moves::{MoveFamily, MoveSet, Turn, MoveTable},
    state::{Bit, CubeState},
};

fn main() {    
    let dimension = 2;
    let rule = MoveSet {
        moves: vec![MoveFamily::Outer, MoveFamily::Rotation, MoveFamily::Wide, MoveFamily::Slice ],
        turns: vec![Turn::Clockwise, Turn::Double, Turn::Anticlockwise],
    };
    let moveset = MoveTable::new(dimension, &rule);
    let mut cube = CubeState::new(dimension);
    
    println!("cube:\nori: {:?} \t perm: {:?}\n", cube.ori[0].to_vec(),cube.perm[0].to_vec());
    let alg = "R F2 R F R U F2 R U";
    for mv in alg.split(' ') {
        moveset.make_move_s(mv, &mut cube);
    }
    println!("alg: \t {}",alg);
    println!("cube:\nori: {:?} \t perm: {:?}\n", cube.ori[0].to_vec(),cube.perm[0].to_vec());
}
