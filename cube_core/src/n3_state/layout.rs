use std::sync::OnceLock;

#[repr(C, align(64))]
#[derive(Debug)]
/// Contains de especific layout of the cube for initializing \
/// must be set before once at the beginning of the program   \
/// and must be set only once.
pub struct Layout {
    /// Contains the init values for the orbits of the cube
    /// (length, init value)
    pub orbit: [(usize, u128); 4],
    /// Number of layers in the cube
    pub n: usize,
    /// Number of orbits in the cube
    pub len: usize,
}

impl Layout {
    /// Creates a new `Layout` from a given number of layers
    fn new(n: usize) -> Self {
        debug_assert!(n > 1, "Expected at least \"2\", got {n}");
        
        let n_mod_2 = n & 1; // n % 2
        let len = (n.pow(2) + 5 * (n & 1) - 2 * n + 4) / 4; // Calculates the number of orbits of given number of layers
        let tmp = (n - 2 - n_mod_2) >> 1; // Temporal value represents de number of pieces between the corner en the edge
        let orbit = if n_mod_2 == 0 {
            [
                (1, 247132686368), // Corner Packed permutation
                (tmp.pow(2) + tmp, 984818244535754528103549039458486304), // 24-pieces orbit packed permutation
                (0, 0),
                (0, 0),
            ]
        } else {
            [
                (1, 247132686368),       // Corner Packed permutation
                (1, 407901468851537952), // Edge Packed permutation
                (1, 172066848),          // Center packed permutation
                (tmp.pow(2) + 2 * tmp, 984818244535754528103549039458486304), // 24-pieces orbit packed permutation
            ]
        };
        Layout { orbit, n, len }
    }
}

/// Contains the specific layout of the cube for initializing \
/// must be set before once at the beginning of the program   \
/// and must be set only once.
pub static GLOBAL_LAYOUT: OnceLock<Layout> = OnceLock::new();

/// Initializes the `GLOBAL_LAYOUT` of the cube with given number of layers
pub fn init_layout(n: usize) {
    debug_assert!(is_none(), "GLOBAL_LAYOUT is initialized");

    GLOBAL_LAYOUT.set(Layout::new(n)).unwrap()
}
/// Returns true if the `GLOBAL_LAYOUT` is not initialized
/// used mostly in debug mode
pub fn is_none() -> bool {
    GLOBAL_LAYOUT.get().is_none()
}
#[inline(always)]
/// Returns the layout of the cube
pub fn get_layout() -> &'static Layout {
    debug_assert!(!is_none(), "GLOBAL_LAYOUT is not initialized");
    GLOBAL_LAYOUT.get().unwrap()
}

mod test {
    use super::*;
    #[test]
    fn init() {
        // par, n = 4
        let layout = Layout::new(4);
        
        assert_eq!(layout.n, 4);
        assert_eq!(layout.len, 3);
        assert_eq!(
            layout.orbit,
            [
                (1, 247132686368),
                (2, 984818244535754528103549039458486304),
                (0, 0),
                (0, 0)
            ]
        );


        // even, n = 5
        let layout = Layout::new(5);
        
        assert_eq!(layout.n, 5);
        assert_eq!(layout.len, 6);
        assert_eq!(
            layout.orbit,
            [
                (1, 247132686368),
                (1, 407901468851537952),
                (1, 172066848),
                (3, 984818244535754528103549039458486304)
            ]
        );
    }
    
    #[test]
    fn set_once() {
        assert!(is_none());
        
        init_layout(4);
        
        assert!(!is_none());
    }
}
