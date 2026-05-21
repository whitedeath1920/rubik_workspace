use cube_core::state::{CubeState, Layout};

fn main() {
    let n  = 7;
    let layout = Layout::new(n);
    let cube =  CubeState::with_layout(&layout);
    
    println!("{}", cube);
}