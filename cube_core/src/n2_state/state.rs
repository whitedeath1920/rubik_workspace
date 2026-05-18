use std::{alloc::{Layout, alloc, handle_alloc_error}, fmt::Display, ops::AddAssign, ptr::{self,NonNull}};

use crate::n2_state::{
    array::Array,
    orbit::{Ori, Perm},
};

use cube_ops::Ops;

#[repr(C)]
#[derive(Ops)]
pub struct Cube2 {
    corner: Ori<0>,
}

#[repr(C)]
#[derive(Ops)]
pub struct Cube3 {
    corner: Ori<0>,
    edge: Ori<1>,
    center: Perm<2>,
}

#[repr(C)]
#[derive(Ops)]
pub struct Cube4 {
    corner: Ori<0>,
    par_center: Perm<3>,
    par_edge: Perm<4>,
}

#[repr(C)]
#[derive(Ops)]
pub struct CubeOdd {
    corner: Ori<0>,
    edge: Ori<1>,
    center: Perm<2>,
    par_center: Array<Perm<3>>,
    par_edge: Array<Perm<4>>,
    edge_center: Array<Perm<5>>,
}
impl CubeOdd {
    #[inline(always)]
    pub fn with_dimension(n: usize) -> NonNull<u8> {
        unsafe {    
            let ptr = Self::ptr().as_ptr() as *mut Self;
            let tmp = (n - 3) >> 1;
            let mut par_center = Array::with_capacity(tmp.pow(2));
            let mut par_edge = Array::with_capacity(tmp);
            let mut edge_center = Array::with_capacity(tmp);
            for i in 0..tmp {
                par_center.write(i, Perm::<3>::default());
                par_edge.write(i,Perm::<4>::default());
                edge_center.write(i,Perm::<5>::default());
            }
            for i in tmp..tmp.pow(2) {
                par_center.write(i,Perm::<3>::default());
            }
            *ptr = Self {
                corner: Ori::<0>::default(),
                edge: Ori::<1>::default(),
                center: Perm::<2>::default(),
                par_center,
                par_edge,
                edge_center,
            };
            NonNull::new_unchecked(ptr as *mut u8)
        }
    }
}
#[repr(C)]
#[derive(Ops)]
pub struct CubePar {
    corner: Ori<0>,
    par_center: Array<Perm<3>>,
    par_edge: Array<Perm<4>>,
}
impl CubePar {
    #[inline(always)]
    pub fn with_dimension(n: usize) -> NonNull<u8> {
        unsafe {
            let layout = Layout::new::<Self>();
            let raw = alloc(layout);
            if raw.is_null() {
                handle_alloc_error(layout);
            }
            let ptr = raw as *mut Self;
            
            
            let tmp = (n - 2) >> 1;
            let mut par_center = Array::with_capacity(tmp.pow(2));
            let mut par_edge = Array::with_capacity(tmp);
            for i in 0..tmp {
                 par_center.write(i, Perm::<3>(128590705839887678326869026826371565600));       
                 par_edge.write(i, Perm::<4>(171126001705004986259790852755342592032));       
                // *p1.add(i) = Perm::<3>::default();
                // *p2.add(i) = Perm::<4>::default();
            }
            for i in tmp..tmp.pow(2) {
                par_center.write(i, Perm::<3>(    128590705839887678326869026826371565600));       
                // *p1.add(i) = Perm::<3>::default();
            }
            ptr::write(ptr, Self {
                corner: Ori::<0>(247132686368,0),
                par_center,
                par_edge,
            });
            
            NonNull::new_unchecked(ptr as *mut u8)
        }
    }
}
#[repr(C)]
pub struct CubeOps {
    to_string: fn(*mut u8) -> String,
    // add: fn(*mut u8, *const u8),
}

pub struct CubeState {
    pub state: NonNull<u8>,
    cubeops: &'static CubeOps,
}

impl CubeState {
    #[inline(always)]
    pub fn new(n: usize) -> Self {
        debug_assert!(n > 1);
        match n {
            2 => CubeState {
                state: Cube2::ptr(),
                cubeops: &OPS_CUBE2,
            },
            3 => CubeState {
                state: Cube3::ptr(),
                cubeops: &OPS_CUBE3,
            },
            4 => CubeState {
                state: Cube4::ptr(),
                cubeops: &OPS_CUBE4,
            },
            n if n & 1 == 0 => CubeState {
                state: CubePar::with_dimension(n),
                cubeops: &OPS_CUBEPAR,
            },
            _ => CubeState {
                state: CubeOdd::with_dimension(n),
                cubeops: &OPS_CUBEODD,
            },
        }
    }
    pub fn _to_string(&self) -> String {
        (self.cubeops.to_string)(self.state.as_ptr())
    }
}
impl Display for CubeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self._to_string())
    }
}
