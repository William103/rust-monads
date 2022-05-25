pub trait Semigroup {
    fn op(self, y: Self) -> Self;
}

// this is quite risky, but should work?
impl<T> Semigroup for T
where
    T: std::ops::Add<Output = T>,
{
    fn op(self, y: Self) -> Self {
        self + y
    }
}
