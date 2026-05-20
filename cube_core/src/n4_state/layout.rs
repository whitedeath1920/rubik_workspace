use core::fmt;

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
/// Contains de especific layout of the cube for initializing \
/// must be set before once at the beginning of the program   \
/// and must be set only once.
pub struct Layout {
    /// Contains the init values for the orbits of the cube
    /// (length, init value)
    pub orbit: [(usize, usize, u128); 5],
    pub len_24: usize,
    /// Number of layers in the cube
    pub n: usize,
    /// Number of orbits in the cube
    pub len: usize,
}

impl Layout {
    /// Creates a new `Layout` from a given number of layers
    pub fn new(n: usize) -> Self {
        assert!(n > 1, "Expected at least \"2\", got {n}");
        let len_24;
        let n_mod_2 = n & 1; // n % 2
        let len = (n.pow(2) + 5 * (n & 1) - 2 * n + 4) / 4; // Calculates the number of orbits of given number of layers
        let tmp = (n - 2 - n_mod_2) >> 1; // Temporal value represents de number of pieces between the corner en the edge
        let orbit = if n_mod_2 == 0 {
            len_24 = tmp.pow(2) + tmp ;
            [
                // (0, 1, 247132686368), // Corner Packed permutation
                (0, 0, 0),
                (0, 0, 0),
                (0, tmp.pow(2), 984818244535754528103549039458486304), // 24-pieces orbit packed permutation
                (tmp.pow(2), tmp, 984818244535754528103549039458486304), // 24-pieces orbit packed permutation
                (0, 0, 0)
            ]
        } else {
            len_24 = tmp.pow(2) + 2*tmp ;
            [
                // (0, 1, 247132686368),       // Corner Packed permutation
                (0, 1, 407901468851537952), // Edge Packed permutation
                (1, 1, 172066848),          // Center packed permutation
                (2,tmp.pow(2),984818244535754528103549039458486304,), // 24-pieces orbit packed permutation
                (2 + tmp.pow(2), tmp,984818244535754528103549039458486304,), // 24-pieces orbit packed permutation
                (2 + tmp.pow(2) + tmp, tmp,984818244535754528103549039458486304,), // 24-pieces orbit packed permutation
            ]
        };
        Layout { orbit, n, len: len - 1, len_24 }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "dimension: {}\tnumber of orbits: {}", self.n, self.len)?;

        Ok(())
    }
}
