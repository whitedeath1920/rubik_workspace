use cube_core::n4_state::{CubeState, with_dimension};

fn main() {
    let n = 100;
    let cube = with_dimension(n);
    println!("{}", cube);
}