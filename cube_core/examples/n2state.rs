use std::{hint::black_box, mem::transmute, time::Instant};

use cube_core::{n_state, n3_state::CubeArena};

// fn test_n3state(n: usize, test: usize, len: usize) {
//     init_layout(n);
//     let time = Instant::now();
//     let mut a = n3_state::CubeState::new();
//     let b = n3_state::CubeState::new();
//     for _ in 0..test {
//         // let _ = black_box(n3_state::CubeState::new());
//         let _ = black_box(a.sub_assign(&b));
//     }
//     println!(
//         "New Version Time taken: {:?} {}",
//         time.elapsed().div_f64(test as f64),
//         n
//     );
//     println!(
//         "New Version Time taken per piece: {:?}ns \t #pieces{}",
//         time.elapsed().as_nanos() as f64 / (test*len) as f64,
//         len
//     )
// }
fn test_cubearena(n: usize, test: usize, len: usize) {
    let time = Instant::now();
    let mut a = CubeArena::with_capacity(n, test);
    // for _ in 0..test {
    //     // let _ = black_box(n3_state::CubeState::new());
    //     // let _ = black_box(a.sub_assign(&b));
    //     let _ = black_box(a.add_assign(0, 1));
    // }
    // println!("length {}",a.states.len());
    // dbg!(&a);
    println!(
        "New Version Time taken: {:?} {}",
        time.elapsed(),//.div_f64(test as f64),
        n
    );
    println!(
        "New Version Time taken per piece: {:?}ns \t#pieces{}",
        time.elapsed().as_nanos() as f64 / (test*len) as f64,
        len
    );
}
// fn check_(n: usize) {
//     init_layout(n);
//     let new = n3_state::CubeState::new();
//     let old = n_state::CubeState::unchecked_new(n);

//     let perm_new = new.orbits.as_slice();
//     let perm_old = old.perm.as_slice();
//     println!("{:?}",(old.perm == new.orbits));
//     let layout = get_layout();
//     for a in 0..layout.len {
//         if perm_new[a] != perm_old[a] {
//             println!("{}\t{}\t{}",a,perm_new[a],perm_old[a]);
//         }
//     }
// }
fn test_1(slc: &u128, len: usize) -> &[u8] {
    let mut arr = Vec::with_capacity(len);
    for a in 0..len {
        arr.push(*slc as u8 >> (a*5));
    }
    
    unsafe { transmute( &arr[..]) }
}
fn main() {    
    let args = std::env::args().collect::<Vec<String>>();
    let n = args[1].parse::<usize>().unwrap();
    let test = args.get(2).unwrap_or(&"1".to_string()).parse::<usize>().unwrap();

    let len = (n.pow(2) + 5 * (n & 1) - 2 * n + 4) / 4;
    // let len = n.pow(3) - (n - 2).pow(3);
    let time = Instant::now();
    let mut a = n_state::CubeState::unchecked_new(n);
    let mut b =  Vec::with_capacity(test);
    for _ in 0..test {
        b.push(n_state::CubeState::unchecked_new(n));
    }
    // for _ in 0..test {
        
    //     // let _ = black_box(state::CubeState::new(n));
    //     let _ = black_box(a.sub_assign(&b[0]));
    // }
    println!(
        "Old Version Time taken: {:?} {}",
        time.elapsed(),//.div_f64(test as f64),
        n
    );
    println!(
        "Old Version Time taken per piece: {:?}ns \t#pieces{}",
        time.elapsed().as_nanos() as f64 / (test * len) as f64,
        len
    );
    test_cubearena(n, test, len);
    // test_n3state(n, test, len);
    // check_(n);
}
