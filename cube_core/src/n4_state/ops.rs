pub trait OpsTrait {
    fn add_assign(&mut self, b: &Self);
    fn add(self, b: &Self) -> Self;
    fn sub_assign(&mut self, b: &Self);
    fn sub(self, b: &Self) -> Self;
}