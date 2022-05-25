use crate::functor::Functor;

pub trait Applicative: Functor {
    fn pure(x: Self::Unwrapped) -> Self;
    fn lift_a2<F, B, C>(self, other: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: Fn(Self::Unwrapped, B) -> C;
}

impl<A> Applicative for Option<A> {
    fn pure(x: A) -> Self {
        Option::Some(x)
    }

    fn lift_a2<F, B, C>(self, other: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: Fn(A, B) -> C,
    {
        use Option::*;
        match (self, other) {
            (Some(x), Some(y)) => Some(f(x, y)),
            _ => None,
        }
    }
}

impl<A, E> Applicative for Result<A, E> {
    fn pure(x: Self::Unwrapped) -> Self {
        Ok(x)
    }

    fn lift_a2<F, B, C>(self, other: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: Fn(Self::Unwrapped, B) -> C,
    {
        match (self, other) {
            (Err(e1), Err(_e2)) => Err(e1),
            (Err(e1), _) => Err(e1),
            (_, Err(e2)) => Err(e2),
            (Ok(x), Ok(y)) => Ok(f(x, y)),
        }
    }
}

