use crate::n4_state::ops::OpsTrait;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Orbit<const KIND: usize>(pub u128);
impl OpsTrait for Orbit<0> {
    #[inline(always)]
    fn add_assign(&mut self, b: &Self) {
        let p = self.0;
        let b = b.0;
        let mut out: u128 = 0;
        for shift in (0..40).step_by(5) {
            let b_block = (b >> shift) & 31;
            let b_perm = (b_block & 7) * 5;
            let b_ori = b_block >> 3;

            let a_block = (p >> b_perm) & 31;
            let a_perm = a_block & 7;
            let a_ori = a_block >> 3;

            let new_ori = ((b_ori + a_ori) % 3) << 3;
            out |= (a_perm | new_ori) << shift;
        }
        self.0 = out;
    }
    #[inline(always)]
    fn add(mut self, b: &Self) -> Self {
        self.add_assign(b);
        self
    }
    #[inline(always)]
    fn sub_assign(&mut self, b: &Self) {
        let p = self.0;
        let b = b.0;
        let mut out = 0;
        for shift in (0..40).step_by(5) {
            let b_block = (b >> shift) & 31;
            let b_perm = (b_block & 7) * 5;
            let b_ori = b_block >> 3;

            let a_block = (p >> shift) & 31;
            let a_perm = a_block & 7;
            let a_ori = a_block >> 3;

            let new_ori = ((a_ori + 3 - b_ori) % 3) << 3;

            out |= (a_perm | new_ori) << b_perm;
        }
        self.0 = out;
    }
    #[inline(always)]
    fn sub(mut self, b: &Self) -> Self {
        self.sub_assign(b);
        self
    }
}
impl OpsTrait for Orbit<1> {
    #[inline(always)]
    fn add_assign(&mut self, b: &Self) {
        let p = self.0;
        let b = b.0;
        let mut out: u128 = 0;
        for shift in (0..60).step_by(5) {
            let b_block = (b >> shift) & 31;
            let b_perm = (b_block & 15) * 5;
            let b_ori = b_block >> 4;

            let a_block = (p >> b_perm) & 31;
            let a_perm = a_block & 15;
            let a_ori = a_block >> 4;

            let new_ori = ((b_ori + a_ori) & 1) << 4;
            out |= (a_perm | new_ori) << shift;
        }
        self.0 = out;
    }
    #[inline(always)]
    fn add(mut self, b: &Self) -> Self {
        self.add_assign(b);
        self
    }
    #[inline(always)]
    fn sub_assign(&mut self, b: &Self) {
        let p = self.0;
        let b = b.0;
        let mut out = 0;
        for shift in (0..60).step_by(5) {
            let b_block = (b >> shift) & 31;
            let b_perm = (b_block & 15) * 5;
            let b_ori = b_block >> 4;

            let a_block = (p >> shift) & 31;
            let a_perm = a_block & 15;
            let a_ori = a_block >> 4;

            let new_ori = ((a_ori + 2 - b_ori) & 2) << 4;

            out |= (a_perm | new_ori) << b_perm;
        }
        self.0 = out;
    }
    #[inline(always)]
    fn sub(mut self, b: &Self) -> Self {
        self.sub_assign(b);
        self
    }
}
impl OpsTrait for Orbit<2> {
    #[inline(always)]
    fn add_assign(&mut self, b: &Self) {
        let p = self.0;
        let b = b.0;
        let mut out = 0u128;
        for shift in (0..30).step_by(5) {
            let b_block = ((b >> shift) & 31) * 5;
            let a_block = (p >> b_block) & 31;
    
            out |= a_block << shift;
        }
        self.0 = out;
    }
    #[inline(always)]
    fn add(mut self, b: &Self) -> Self {
        self.add_assign(b);
        self
    }
    #[inline(always)]
    fn sub_assign(&mut self, b: &Self) {
        let p = self.0;
        let b = b.0;
        let mut out = 0;
        for shift in (0..30).step_by(5) {
            let b_block = ((b >> shift) & 31) * 5;
            let a_block = (p >> shift) & 31;
    
            out |= a_block << b_block;
        }
        self.0 = out;
    }
    #[inline(always)]
    fn sub(mut self, b: &Self) -> Self {
        self.sub_assign(b);
        self
    }
}

impl<const KIND: usize> Default for Orbit<KIND> {
    fn default() -> Self {
        match KIND {
            0 => Self(247132686368),
            1 => Self(42535295865117307933329727397822564384),
            2 => Self(85070591730234615865843651858114119712),
            3 => Self(128590705839887678326869026826371565600),
            4 => Self(171126001705004986259790852755342592032),
            5 => Self(213661297570122294192712678684313618464),
            _ => panic!("KIND must be 0 or 1 (corner or 12 edge)"),
        }
    }
}
