use std::ops::{Add, AddAssign, Sub, SubAssign};

use cube_ops::Ops;

use crate::n4_state::{ops::OpsTrait, orbit::Orbit};

pub trait CubeBehavior: Default + std::fmt::Display {
    // fn add_assign(&mut self, b: &Self);
    // fn add(self, b: &Self) -> Self;
    // fn sub_assign(&mut self, b: &Self);
    // fn sub(self, b: &Self) -> Self;
}
#[derive(Debug, Default, Ops)]
pub struct Cube2 {
    corner: Orbit<0>,
}
#[derive(Debug, Default, Ops)]
pub struct Cube3 {
    corner: Orbit<0>,
    edge: Orbit<1>,
    center: Orbit<2>,
}
#[derive(Debug, Default, Ops)]
pub struct Cube4 {
    corner: Orbit<0>,
    par_center: Orbit<3>,
    par_edge: Orbit<4>,
}
#[derive(Debug, Default, Ops)]
pub struct CubeOdd {
    corner: Orbit<0>,
    edge: Orbit<1>,
    center: Orbit<2>,
    par_center: Vec<Orbit<3>>,
    par_edge: Vec<Orbit<4>>,
    edge_center: Vec<Orbit<5>>,
}
impl CubeOdd {
    pub fn with_dimension(n: usize) -> Self {
        let tmp = (n - 3) >> 1;
        let mut par_center = Vec::with_capacity(tmp.pow(2));
        let mut par_edge = Vec::with_capacity(tmp);
        let mut edge_center = Vec::with_capacity(tmp);
        for i in 0..tmp {
            par_center.push(Orbit::<3>::default());
            par_edge.push(Orbit::<4>::default());
            edge_center.push(Orbit::<5>::default());
        }
        for i in tmp..tmp.pow(2) {
            par_center.push(Orbit::<3>::default());
        }
        Self {
            corner: Orbit::<0>::default(),
            edge: Orbit::<1>::default(),
            center: Orbit::<2>::default(),
            par_center,
            par_edge,
            edge_center,
        }
    }
}
#[derive(Debug, Default, Ops)]
pub struct CubePar {
    corner: Orbit<0>,
    par_center: Vec<Orbit<3>>,
    par_edge: Vec<Orbit<4>>,
}
impl CubePar {
    pub fn with_dimension(n: usize) -> Self {
        let tmp = (n - 2) >> 1;
        let mut par_center = Vec::with_capacity(tmp.pow(2));
        let mut par_edge = Vec::with_capacity(tmp);
        for _ in 0..tmp {
            par_center.push(Orbit::<3>(128590705839887678326869026826371565600));
            par_edge.push(Orbit::<4>(171126001705004986259790852755342592032));
            // *p1.add(i) = Orbit::<3>::default();
            // *p2.add(i) = Orbit::<4>::default();
        }
        for _ in tmp..tmp.pow(2) {
            par_center.push(Orbit::<3>(128590705839887678326869026826371565600));
            // *p1.add(i) = Perm::<3>::default();
        }
        Self {
            corner: Orbit::<0>::default(),
            par_center,
            par_edge,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CubeState<C: CubeBehavior> {
    inner: C,
}
#[derive(Debug)]
pub enum DynamicCubeState {
    Two(CubeState<Cube2>),
    Three(CubeState<Cube3>),
    Four(CubeState<Cube4>),
    Odd(CubeState<CubeOdd>),
    Par(CubeState<CubePar>),
}
impl std::fmt::Display for DynamicCubeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicCubeState::Two(cube) => write!(f, "{}", cube),
            DynamicCubeState::Three(cube) => write!(f, "{}", cube),
            DynamicCubeState::Four(cube) => write!(f, "{}", cube),
            DynamicCubeState::Odd(cube) => write!(f, "{}", cube),
            DynamicCubeState::Par(cube) => write!(f, "{}", cube),
        }
    }
}
pub fn with_dimension(n: usize) -> DynamicCubeState {
    match n {
        0..=1 => panic!("dimension must be greater than 1"),
        2 => DynamicCubeState::Two(CubeState {
            inner: Cube2::default(),
        }),
        3 => DynamicCubeState::Three(CubeState {
            inner: Cube3::default(),
        }),
        4 => DynamicCubeState::Four(CubeState {
            inner: Cube4::default(),
        }),
        _ => {
            if n % 2 == 0 {
                DynamicCubeState::Par(CubeState {
                    inner: CubePar::with_dimension(n),
                })
            } else {
                DynamicCubeState::Odd(CubeState {
                    inner: CubeOdd::with_dimension(n),
                })
            }
        }
    }
}
impl<C: CubeBehavior + std::fmt::Display> std::fmt::Display for CubeState<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)?;
        Ok(())
    }
}
impl<C: CubeBehavior + Default> Default for CubeState<C> {
    fn default() -> Self {
        Self {
            inner: <C as Default>::default(),
        }
    }
}
impl<C: CubeBehavior + OpsTrait> AddAssign<&CubeState<C>> for CubeState<C> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        self.inner.add_assign(&rhs.inner);
    }
}
impl<C: CubeBehavior + OpsTrait> Add<&CubeState<C>> for CubeState<C> {
    type Output = Self;
    fn add(mut self, rhs: &CubeState<C>) -> Self::Output {
        self.inner.add_assign(&rhs.inner);
        self
    }
}
impl<C: CubeBehavior + OpsTrait> SubAssign<&CubeState<C>> for CubeState<C> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Self) {
        self.inner.sub_assign(&rhs.inner);
    }
}
impl<C: CubeBehavior + OpsTrait> Sub<&CubeState<C>> for CubeState<C> {
    type Output = Self;
    fn sub(mut self, rhs: &CubeState<C>) -> Self::Output {
        self.inner.sub_assign(&rhs.inner);
        self
    }
}
