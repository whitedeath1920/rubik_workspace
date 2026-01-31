use cube_core::{array_build, n_state::{Bit, for_nested, from_perm_to_slice}};

#[test]
fn array_build() {
    let a = array_build!(mask_set: u32, 2, 5);
    let b = [
        !3, !12, !48, !192, !768, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(a, b);

    let a = array_build!(shift: u32, 2, 5);
    let b = [
        0, 2, 4, 6, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(a, b);
}
#[test]
fn get_set() {
    let mut a: u128 = 0;
    a.set(3, 4);
    assert_eq!(4, a.get(3));
}
#[test]
fn get_set_kind() {
    let mut a: u128 = 0;
    a.set_kind(5);
    assert_eq!(5 << 125, a);
    assert_eq!(5, a.get_kind());
}
#[test]
fn slice_vec() {
    let slice = [0, 1, 2, 3, 4, 5, 6, 7, 0];
    let a = u128::from_slice(&slice);
    assert_eq!(a, 247132686368);

    let mut b = [0; 25];
    a.to_slice(&mut b);
    assert_eq!(
        b,
        [
            0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
    
    assert_eq!(a.to_vec(), slice.to_vec());
}
#[test]
fn nested() {
    
    for_nested();
    
    assert!(false);
}