use cube_core::state::CubeState;

#[test]
fn test_new() {
    let dimension = 7;
    let cube = CubeState::new(dimension);

    let perm = [
        247132686368,
        42535295865117307933329727397822564384,
        85070591730234615865843651858114119712,
        128590705839887678326869026826371565600,
        128590705839887678326869026826371565600,
        171126001705004986259790852755342592032,
        171126001705004986259790852755342592032,
        213661297570122294192712678684313618464,
        213661297570122294192712678684313618464,
        256196593435239602125634504613284644896,
        256196593435239602125634504613284644896,
    ];
    let ori = [0, 536870912];

    assert_eq!(cube.perm, perm);
    assert_eq!(cube.ori, ori);
}

#[test]
fn test_to_vec() {
    let dimension = 7;
    let cube = CubeState::new(dimension);
    let vect = cube.to_vec();

    let perm = vec![
        vec![0, 1, 2, 3, 4, 5, 6, 7, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 3,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 3,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 4,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 4,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 5,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 5,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 6,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 6,
        ],
    ];

    let ori = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];

    assert_eq!(vect.0, perm);
    assert_eq!(vect.1, ori);
}

#[test]
fn test_from_vec() {
    let dimension = 7;
    let cube = CubeState::new(dimension);

    let perm = vec![
        vec![0, 1, 2, 3, 4, 5, 6, 7, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 3,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 3,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 4,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 4,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 5,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 5,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 6,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 6,
        ],
    ];

    let ori = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];

    let cube_vect = CubeState::from_vec((perm, ori));

    assert_eq!(cube, cube_vect);
}
#[test]
fn test_add() {
    let dimension = 3;
    let cube0 = CubeState::new(dimension);

    let perm1 = vec![
        vec![0, 1, 2, 3, 5, 6, 7, 4, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori1 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube1 = CubeState::from_vec((perm1, ori1));

    let perm2 = vec![
        vec![0, 1, 3, 7, 4, 5, 2, 6, 0],
        vec![0, 1, 7, 3, 4, 5, 2, 10, 8, 9, 6, 11, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori2 = vec![
        vec![0, 0, 1, 2, 0, 0, 2, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube2 = CubeState::from_vec((perm2, ori2));

    let perm3 = vec![
        vec![0, 1, 3, 4, 5, 6, 2, 7, 0],
        vec![0, 1, 7, 3, 4, 5, 2, 11, 9, 10, 6, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori3 = vec![
        vec![0, 0, 1, 2, 0, 0, 2, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube3 = CubeState::from_vec((perm3, ori3));

    assert_eq!(cube1, cube0.clone() + &cube1);
    assert_eq!(cube1, cube1.clone() + &cube0);
    assert_eq!(
        cube0,
        cube1.clone() + &cube1 + &cube1 + &cube1
    );
    assert_eq!(cube3, cube1 + &cube2);
}
#[test]
fn test_sub() {
    let dimension = 3;
    let cube0 = CubeState::new(dimension);

    let perm1 = vec![
        vec![0, 1, 2, 3, 5, 6, 7, 4, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori1 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube1 = CubeState::from_vec((perm1, ori1));

    let perm2 = vec![
        vec![0, 1, 3, 7, 4, 5, 2, 6, 0],
        vec![0, 1, 7, 3, 4, 5, 2, 10, 8, 9, 6, 11, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori2 = vec![
        vec![0, 0, 1, 2, 0, 0, 2, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube2 = CubeState::from_vec((perm2, ori2));

    let perm3 = vec![
        vec![0, 1, 3, 4, 5, 6, 2, 7, 0],
        vec![0, 1, 7, 3, 4, 5, 2, 11, 9, 10, 6, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori3 = vec![
        vec![0, 0, 1, 2, 0, 0, 2, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube3 = CubeState::from_vec((perm3, ori3));

    let perm4 = vec![
        vec![0, 1, 2, 3, 7, 4, 5, 6, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori4 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube4 = CubeState::from_vec((perm4, ori4));

    assert_eq!(cube4.clone(), cube0.clone() - &cube1);
    assert_eq!(cube1, cube1.clone() - &cube0);
    assert_eq!(cube0, cube1.clone() - &cube1);
    assert_eq!(cube1, cube3.clone() - &cube2);
}
#[test]
fn test_identity() {
    let dimension = 3;
    let cube0 = CubeState::new(dimension);

    assert_eq!(cube0.clone(), cube0.identity());
}
#[test]
fn test_inverse() {
    let dimension = 3;
    let cube0 = CubeState::new(dimension);

    let perm1 = vec![
        vec![0, 1, 2, 3, 5, 6, 7, 4, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori1 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube1 = CubeState::from_vec((perm1, ori1));

    let perm4 = vec![
        vec![0, 1, 2, 3, 7, 4, 5, 6, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori4 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube4 = CubeState::from_vec((perm4, ori4));

    assert_eq!(cube0.clone() - &cube1, -cube1.clone());
    assert_eq!(-cube1.clone(), cube4);
    assert_eq!(cube1, -cube4);
    assert_eq!(cube0, cube1.clone() + &cube1.invert());
}
#[test]
fn test_mul() {
    let dimension = 3;
    let cube0 = CubeState::new(dimension);

    let perm1 = vec![
        vec![0, 1, 2, 3, 5, 6, 7, 4, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori1 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube1 = CubeState::from_vec((perm1, ori1));

    assert_eq!(cube0, 4 * cube1.clone());
    assert_eq!(4 * cube1.clone(), 4 * cube1.clone());
    assert_eq!(cube0, 0 * cube1.clone());
    assert_eq!(cube1.clone(), 1 * cube1.clone());
    assert_eq!(-cube1.clone(), -1 * cube1.clone());
    
}
#[test]
fn test_eq() {
    let perm1 = vec![
        vec![0, 1, 2, 3, 4, 5, 6, 7, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 3,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 3,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 4,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 4,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 5,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 5,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 6,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 6,
        ],
    ];
    let ori = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];

    let cube1 = CubeState::from_vec((perm1, ori));

    let perm2 = vec![
        vec![0, 1, 2, 3, 4, 5, 6, 7, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
        vec![
            1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 3,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 3,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 4,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 4,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 5,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 5,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 6,
        ],
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 6,
        ],
    ];
    let ori = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube2 = CubeState::from_vec((perm2, ori));

    assert_eq!(cube1, cube2);
}

#[test]
fn test_check() {
    let dimension = 7;
    let cube = CubeState::new(dimension);

    let perm1 = vec![
        vec![0, 1, 2, 3, 5, 6, 7, 4, 0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori1 = vec![
        vec![0, 1, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube1 = CubeState::from_vec((perm1, ori1));

    let perm2 = vec![
        vec![0, 1, 2, 3, 5, 6, 7, 4, 0],
        vec![0, 2, 1, 3, 4, 5, 6, 7, 9, 10, 11, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2],
    ];
    let ori2 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let cube2 = CubeState::from_vec((perm2, ori2));

    assert!(cube.check().is_ok());
    assert!(cube1.check().is_ok());
    assert!(cube2.check().is_err());
}

#[test]
fn test_get_modulus() {
    let perm1 = vec![vec![0, 1, 3, 7, 5, 2, 6, 4, 0]];
    let ori1 = vec![
        vec![0, 0, 1, 2, 0, 0, 2, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let a = CubeState::from_vec((perm1, ori1));
    
    let perm2= vec![
        vec![0, 1, 3, 7, 5, 2, 6, 4, 0],
        vec![0, 1, 7, 3, 4, 5, 2, 10, 9, 6, 11, 8, 1],
        vec![0, 1, 2, 3, 4, 5, 2]
    ];
    let ori2= vec![
        vec![0, 0, 1, 2, 0, 0, 2, 1, 0],
        vec![0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1],
    ];
    let b = CubeState::from_vec((perm2, ori2));
    
    assert_eq!(a.get_modulus()*a.clone(), a.identity());
    assert_eq!(b.get_modulus()*b.clone(), b.identity());
}

#[test]
fn test_opers() {
    let dimension = 3;
    let cube1 = CubeState::new(dimension);
    let cube2 = cube1.clone();

    assert_eq!(cube1, cube1.clone() + &cube1);
    assert_eq!(cube2, 3*cube1.clone());
    assert_eq!(cube2, -3*-&cube1);
}