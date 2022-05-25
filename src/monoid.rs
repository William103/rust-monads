use crate::{semigroup::Semigroup, list::List};

pub trait Monoid: Semigroup {
    fn identity() -> Self;

    fn concat(ls: List<Self>) -> Self
    where
        Self: Sized,
    {
        let f = |x: Self, y: Self| -> Self { x.op(y) };

        ls.foldl(&f, Self::identity())
    }
}

