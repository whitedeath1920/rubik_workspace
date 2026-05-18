use std::{hint::black_box, time, vec};

use cube_core::n3_state::{CubeArena, state::{add_8, add_24, sub_8, sub_24}};

fn to_vec(mut a: u128) -> Vec<(u8,u8)> {
    let mut out = Vec::with_capacity(8);
    for _ in 0..8 {
        let block = a & 31;
        let perm = block & 7;
        let ori = block >> 3;
        out.push((perm as u8, ori as u8));
        
        a >>= 5;
    }
    out
}
fn to_bit(arr: &[(u8,u8)]) -> u128 {
    let mut out = 0;
    for (i, (perm, ori)) in arr.iter().enumerate() {
        out |= ((perm | (ori << 3)) as u128) << (i * 5);
    }
    out
}
fn check() {
    let zero = vec![(0,0),(1,0),(2,0),(3,0),(4,0),(5,0),(6,0),(7,0)];
    let u = vec![(0,0),(1,0),(2,0),(3,0),(5,0),(6,0),(7,0),(4,0)];
    let r = vec![(0,0),(1,0),(3,1),(7,2),(4,0),(5,0),(2,2),(6,1)];
    let mut u_r = to_bit(&u);
    add_8(&mut u_r, to_bit(&r));
    println!("{:?}",to_vec(u_r));
    
    let mut r_u_inv = to_bit(&zero);
    sub_8(&mut r_u_inv, to_bit(&r));
    sub_8(&mut r_u_inv, to_bit(&u));
    println!("{:?}",to_vec(r_u_inv));
    
    let mut cl = r_u_inv.clone();
    add_8(&mut cl, u_r);
    println!("{:?}",to_vec(cl));

    add_8(&mut u_r, r_u_inv);
    println!("{:?}",to_vec(u_r));

    assert_eq!(u_r, cl);
}
fn add_sub_benchmark() {
    let test: usize = 4_000_000_000;
    let time = time::Instant::now();
    for _ in 0..test {
        let _ = black_box(add_8(&mut 111087323688,1234123411));
    }
    println!("Add Time elapsed: {:?}",time.elapsed());
    println!("Time per cubie:   {:?}ns",time.elapsed().as_nanos() as f64 / ((24*test) as f64));

    let time = time::Instant::now();
    for _ in 0..test {
        let _ = black_box(sub_8(&mut 111087323688, 601919798700));        
    }
    println!("Sub Time elapsed: {:?}",time.elapsed());
    println!("Time per cubie:   {:?}ns",time.elapsed().as_nanos() as f64 / ((24*test) as f64));
}
fn new_benchmark() {
    let test: usize = 400_000;
    let time = time::Instant::now();
    let a = CubeArena::zeros(1000, test);

    
    println!("New Benchmark Time elapsed: {:?}",time.elapsed());
    println!("Time per orbit: {:?}ns",time.elapsed().as_nanos() as f64 / (test) as f64);
    
}
fn add_bench() {
    let test: usize = 4000;
    let time = time::Instant::now();

    
    let mut arena = CubeArena::zeros(100, 2);
    // arena.insert(0, &[111087323688]);
    // arena.insert(1, &[601919798700]);
    for _ in 0..test {
        let _ = black_box(arena.add_assign(0,1));
    }
    println!("Add Benchmark Time elapsed: {:?}",time.elapsed());
    println!("Time per orbit:   {:?}ns",time.elapsed().as_nanos() as f64 / ((test) as f64));
}
fn main() {
    add_bench();
}
