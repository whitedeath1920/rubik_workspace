use cube_core::n_state::Bit;
use std::time::SystemTime;
use std::{collections::HashMap, fmt::Debug, hash::Hash, vec};
use std::hint::black_box;

fn factorial(m: usize) -> usize {
    let mut fact = 1;
    for a in 1..=m {
        fact *= a;
    }
    fact
}
fn check_perms<T: Eq + Hash + Debug>(perms: &[T]) -> bool {
    let mut cont = 0;
    let mut hs = HashMap::new();
    for (i, perm) in perms.iter().enumerate() {
        if let Some(idx) = hs.insert(perm, i) {
            println!("value: {:?} repeated in old:{}, new:{}", perm, idx, i);
            cont += 1;
        }
    }
    println!("Total repeated: {}", cont);
    return cont == 0;
}
fn desfaze(des: usize, arr: &mut [usize]) {
    let mut ac = Vec::new();
    arr.clone_into(&mut ac);
    let m = arr.len();
    for i in 0..m {
        arr[(des + i) % m] = ac[i];
    }
}

fn n_perm_(n: usize, m: usize) -> Vec<usize> {
    let mut perm = Vec::from_iter(0..m);
    for g in 2..=m {
        let mut ratio = 1;
        for a in (g + 1)..=m {
            ratio *= a;
        }
        let step = ((n - (n % ratio)) / ratio) % g;
        desfaze(step, &mut perm[(m - g)..m]);
    }
    perm
}

#[inline(always)]
fn select_nth(mut bits: u64, mut n: u128) -> u32 {
    while n != 0 {
        bits &= bits - 1;
        n -= 1;
    }
    return bits.trailing_zeros();
}

fn get_all_k_perms<const PERM: usize, const LENGTH: usize, const K: usize>()
-> ([[u8; LENGTH]; PERM], [u128; PERM]) {
    debug_assert!(LENGTH <= K);
    let total = (0..LENGTH).fold(1, |acc, i| acc * (K - i)) as u128;
    let mut perms = [[0; LENGTH]; PERM];
    let mut b_perms = [0; PERM];
    (0..total)
        .zip(&mut perms)
        .zip(&mut b_perms)
        .for_each(|((mut n, perm), b_perm)| {
            let mut bits = (1 << K) - 1;
            (0..LENGTH).for_each(|i| {
                let block = (0..LENGTH - 1 - i).fold(1, |acc, j| acc * (K - i - 1 - j)) as u128;
                let idx = n / block;
                n %= block;
                let pos = select_nth(bits, idx);
                perm[i] = pos as u8;
                *b_perm |= (pos as u128) << (i * 5);
                bits &= !(1 << pos);
            });
        });

    (perms, b_perms)
}
fn build_dict<const PERM: usize, const LENGTH: usize>(
    perms: [[u8; LENGTH]; PERM],
    b_perms: [u128; PERM],
) -> HashMap<(u128, u128), u128> {
    let mut p = HashMap::with_capacity(PERM * PERM);
    for i in 0..PERM {
        for j in 0..PERM {
            let mut tmp: u128 = 0;
            for k in 0..LENGTH {
                tmp |= (perms[j][k] as u128) << (perms[i][k] * 5);
            }
            p.insert((b_perms[j], b_perms[i]), tmp);
        }
    }
    p
}
fn build_vect<const PERM: usize, const LENGTH: usize>(
    perms: [[u8; LENGTH]; PERM],
    b_perms: [u128; PERM],
) -> Vec<(u128,Vec<(u128,u128)>)> {
    let mut p = Vec::with_capacity(PERM);
    for i in 0..PERM {
        let mut vec = Vec::with_capacity(PERM);
        for j in 0..PERM {
            let mut tmp: u128 = 0;
            for k in 0..LENGTH {
                tmp |= (perms[j][k] as u128) << (perms[i][k] * 5);
            }
            vec.push((b_perms[j], tmp));
        }
        p.push((b_perms[i],vec));
    }
    p
}
fn lookup(data: &[(u128,Vec<(u128,u128)>)], key: (u128,u128)) -> &u128 {
    let i = data.binary_search_by_key(&key.0, |(k,_)| *k)
        .unwrap();
    let j = data[i].1.binary_search_by_key(&key.1, |&(k,_)| k)
        .unwrap();
    &data[i].1[j].1
}
fn permute_8(mut a: u128,mut b: u128, dict: &HashMap<(u128, u128), u128>) -> u128 {
    let mut r = a.get_kind() as u128;
    let mask = (1<<(4*5)) - 1;
    r |= dict.get(&(a & mask, b & mask)).unwrap();
    a = a >> 4*5;
    b = b >> 4*5;
    r |= dict.get(&(a & mask, b & mask)).unwrap();
    r
}
fn permute_8_v(mut a: u128,mut b: u128, dict: &[(u128, Vec<(u128, u128)>)]) -> u128 {
    let mut r = a.get_kind() as u128;
    let mask = (1<<(4*5)) - 1;
    r |= lookup(dict, (a & mask,b & mask));
    a = a >> 4*5;
    b = b >> 4*5;
    r |= lookup(dict, (a & mask, b & mask));
    r
}
fn permute_12(mut a: u128,mut b: u128, dict: &HashMap<(u128, u128), u128>) -> u128 {
    let mut r = a.get_kind() as u128;
    let mask = (1<<(3*5)) - 1;
    r |= dict.get(&(a & mask, b & mask)).unwrap();
    a = a >> 3*5;
    b = b >> 3*5;
    r |= dict.get(&(a & mask, b & mask)).unwrap();
    a = a >> 3*5;
    b = b >> 3*5;
    r |= dict.get(&(a & mask, b & mask)).unwrap();
    r
}
fn permute_6(a: u128,b: u128, dict: &HashMap<(u128, u128), u128>) -> u128 {
    *dict.get(&(a, b)).unwrap()
}
fn main() {
    const K: usize = 8;
    const LENGTH: usize = 4;
    const PERM: usize = {
        let mut r = 1;
        let mut i = 0;
        while i < LENGTH {
            r *= K - i;
            i += 1;
        }
        r
    };
    let (perms, b_perms) = get_all_k_perms::<PERM, LENGTH, K>();

    let dict = build_dict::<PERM, LENGTH>(perms, b_perms);
    // let dict = build_vect::<PERM, LENGTH>(perms, b_perms);
    println!("dict len: {}", dict.len());
    let a = u128::from_slice(&[4,5,1,6,2,7,0,3,0]);
    let b = u128::from_slice(&[7,3,0,1,6,2,4,5,0]);
    // let a = u128::from_slice(&[4,5,1,6,2,7,11,0,3,10,8,9,1]);
    // let b = u128::from_slice(&[6,2,1,7,11,4,0,3,10,8,9,5,1]);
    // let a = u128::from_slice(&[4,5,1,6,2,0,3]);
    // let b = u128::from_slice(&[3,4,1,2,5,0,3]);
    let init = SystemTime::now();
    let num = 100_000u128;
    for _ in 0..num {
        black_box(permute_8(a,b,&dict));
    }
    println!("Time elapsed: {:?}", init.elapsed().unwrap());
    println!("operations: {:?} op/s", num as f64/ init.elapsed().unwrap().as_secs_f64());
    println!("operations: {:?}", (init.elapsed().unwrap().div_f64(num as f64)));
}
fn main_() {
    let m: usize = 10;
    assert!(m <= 24);
    let fact = factorial(m);
    let mut perms = vec![vec![]; fact];
    let mut mods = vec![0; fact];
    for n in 0..fact {
        perms[n] = n_perm_(n, m);
        // println!("{}\t{:?}", n, n_perm(n, m));
    }
    check_perms(&perms);
}
// fn main() {
//     let m: usize = 9; // modulus of the permutation
//     let fact = factorial(m);
//     let total = 2*m;
//     let mut perms = vec![vec![m+1;m];total];
//     for n in 0..total {
//         for num in 0..m {
//             perms[n][idx(n,m,num)] = num;
//         }
//         if n%m == 0 {
//             println!("------------");
//             println!("step:{}",step(n,m));
//         }
//         println!("{}:\t{:?}",n,perms[n]);
//     }
//     if check_perms(&perms) {
//         println!("All permutations are unique");
//     }
// }
