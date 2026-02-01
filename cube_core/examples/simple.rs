#![feature(new_range_api)]
#![allow(dead_code, unused_imports)]

use std::{
    env::{self, Args},
    fmt::format,
    io::{self, BufWriter, Stdout, StdoutLock, Write},
    ops::Range,
    time::{self, Duration, SystemTime},
};

use cube_core::{
    moves::{CubePair, MoveFamily, MoveSet, MoveTable, Turn},
    n_state,
    state::{Bit, CubeState},
};
enum Layer {
    X(usize),
    Y(usize),
    Z(usize),
}
impl Layer {
    fn get(&self, len: usize) -> (Range<usize>, Range<usize>, Range<usize>) {
        match self {
            Layer::X(layer) => {
                assert!(*layer < len);
                (*layer..*layer + 1, 0..len, 0..len)
            }
            Layer::Y(layer) => {
                assert!(*layer < len);
                (0..len, *layer..*layer + 1, 0..len)
            }
            Layer::Z(layer) => {
                assert!(*layer < len);
                (0..len, 0..len, *layer..*layer + 1)
            }
        }
    }
    fn to_string(&self) -> String {
        match self {
            Layer::X(i) => format!("x: {i}"),
            Layer::Y(i) => format!("y: {i}"),
            Layer::Z(i) => format!("z: {i}"),
        }
    }
}
fn get_layer(axis: Layer, arr: &Vec<Vec<Vec<u16>>>) -> Vec<([u16; 3], u16)> {
    let (x, y, z) = axis.get(arr.len());
    let mut arr2 = vec![([0, 0, 0], 0); arr.len().pow(2)];
    let mut count = 0;
    for i in x {
        for j in y.clone() {
            for k in z.clone() {
                arr2[count] = ([i as u16, j as u16, k as u16], arr[i][j][k]);
                count += 1;
            }
        }
    }
    arr2
}
fn traverse_cube(n: usize, arr: &Vec<Vec<Vec<u16>>>, full: bool) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());
    let mut layer = vec![([0u16; 3], 0); n.pow(2)];
    let axis = [Layer::X, Layer::Y, Layer::Z];
    for i in 0..n {
        for a in axis {
            layer = get_layer(a(i), arr); // Selecciona la capa respecto al eje seleccionado
            print_layer(&mut handle,&layer, a(i), full)?;
        }
    }
    handle.flush()?;

    Ok(())
}
fn create_matrix(n: usize) -> Vec<Vec<Vec<u16>>> {
    let mut a = vec![vec![vec![0; n]; n]; n];
    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                a[i][j][k] = count;
                count += 1;
            }
        }
    }
    a
}
fn print_layer(handle:&mut BufWriter<StdoutLock>,layer: &Vec<([u16; 3], u16)>, layer_id: Layer, full: bool) -> io::Result<()> {
    writeln!(handle, "\nlayer:\t{}", layer_id.to_string())?;
    if full {
        for f in layer {
            writeln!(handle, "{:?}:\t{}", f.0, f.1)?
        }
    }
    writeln!(handle, "count: {}", layer.len())?; // layer.len() == n.pow(2)

    Ok(())
}
fn total_layer(n: usize) -> f64 {
    (3 * n) as f64
}
fn total_pieces(n: usize) -> u128 {
    (n.pow(3) * 3) as u128
}
fn add() {
    let dimension = 3;
    let cube0 = n_state::CubeState::unchecked_new(dimension);

    let perm1 = vec![
        vec![0, 1, 2, 3, 5, 6, 7, 4, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori1 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube1 = n_state::CubeState::from_vec((perm1, ori1));
    dbg!(&cube1);
    
    let perm2 = vec![
        vec![0, 1, 3, 7, 4, 5, 2, 6, 0],
        vec![0, 1, 7, 3, 4, 5, 2, 10, 8, 9, 6, 11, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori2 = vec![
        vec![0, 0, 1, 2, 0, 0, 2, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube2 = n_state::CubeState::from_vec((perm2, ori2));

    let perm3 = vec![
        vec![0, 1, 3, 4, 5, 6, 2, 7, 0],
        vec![0, 1, 7, 3, 4, 5, 2, 11, 9, 10, 6, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori3 = vec![
        vec![0, 0, 1, 2, 0, 0, 2, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube3 = n_state::CubeState::from_vec((perm3, ori3));

    // let tmp = cube0.clone() + &cube1;
    // for (a,b) in tmp.perm.iter().zip(cube1.perm.iter()) {
    //     assert_eq!(a.to_vec(),b.to_vec());
    // }
    assert_eq!(cube1, cube0.clone() + &cube1);
    assert_eq!(cube1, cube1.clone() + &cube0);
    assert_eq!(cube0, cube1.clone() + &cube1 + &cube1 + &cube1);
    assert_eq!(cube1, cube1.clone() + &cube0);
    assert_eq!(cube3, cube1.clone() + &cube2);
}
fn main() -> io::Result<()> {
    add();
    return Ok(());
    let args = env::args().collect::<Vec<String>>();
    assert!(args.len() == 2);
    let n: usize = args[1].parse().unwrap();
    println!("Creating nxnxn, n = {n}");
    let cubepair = CubePair::new(n);
    cubepair.print();
    // let arr = create_matrix(n); // Crea una matriz donde cada elemento solo guarda su correlativo
    // println!("Traversing array");
    // let init = SystemTime::now();
    // traverse_cube(n, &arr, false)?;
    // let delta = init.elapsed().unwrap();
    // println!("time elapsed: {:?}", delta);
    // println!(
    //     "analized: \nTotal layers:{}\t\t{} layer/s\nTotal pieces:{}\t\t {} ns/piece",
    //     total_layer(n),
    //     total_layer(n) / delta.as_secs_f64(),
    //     total_pieces(n),
    //     delta.as_nanos()/total_pieces(n)
    // );
    Ok(())
}





fn _asdf() {
    let dimension = 2;
    let rule = MoveSet {
        moves: vec![
            MoveFamily::Outer,
            MoveFamily::Rotation,
            MoveFamily::Wide,
            MoveFamily::Inner,
        ],
        turns: vec![Turn::Clockwise, Turn::Double, Turn::Anticlockwise],
    };
    let moveset = MoveTable::new(dimension, &rule);
    let mut cube = CubeState::new(dimension);

    println!(
        "cube:\nori: {:?} \t perm: {:?}\n",
        cube.ori[0].to_vec(),
        cube.perm[0].to_vec()
    );
    let alg = "R F2 R F R U F2 R U";
    for mv in alg.split(' ') {
        moveset.make_move_s(mv, &mut cube);
    }
    println!("alg: \t {}", alg);
    println!(
        "cube:\nori: {:?} \t perm: {:?}\n",
        cube.ori[0].to_vec(),
        cube.perm[0].to_vec()
    );
}
