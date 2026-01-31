use cube_core::n_state::Array;

#[test]
fn slice() {
    let slice = [0, 1, 2, 3];
    let arr: Array = Array::from_slice(&slice);

    assert_eq!(slice, arr.as_slice());
}

#[test]
fn mut_slice() {
    let slice = [0, 1, 2, 3];
    let mut arr: Array = Array::from_slice(&slice);   
    let b = arr.as_mut_slice();
    b[2] = 44;

    assert_eq!(b, [0, 1, 44, 3]);
}

#[test]
fn get_set() {
    let mut arr: Array = Array::with_capacity(3); 
    unsafe { 
        *arr.as_mut_ptr().add(2) = 123;
        assert_eq!(*arr.as_ptr().add(2), 123);
    }
}

#[test]
fn zeroed() {
    let arr: Array = Array::zeroed(4);
    assert_eq!([0; 4], arr.as_slice());
}

#[test]
fn iter() {
    let slice = [123423134, 1, 2, 3];
    let arr: Array = Array::from_slice(&slice);

    for (a, b) in arr.iter().zip(slice) {
        assert_eq!(*a, b);
    }
}