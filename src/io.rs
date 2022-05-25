use crate::{applicative::Applicative, functor::Functor, monad::Monad};

pub struct IO<A> {
    value: A,
}

impl<A> Functor for IO<A> {
    type Unwrapped = A;
    type Wrapped<B> = IO<B>;

    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(A) -> B,
    {
        IO {
            value: f(self.value),
        }
    }
}

impl<A> Applicative for IO<A> {
    fn pure(x: Self::Unwrapped) -> Self {
        IO { value: x }
    }

    fn lift_a2<F, B, C>(self, other: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: Fn(Self::Unwrapped, B) -> C,
    {
        IO {
            value: f(self.value, other.value),
        }
    }
}

impl<A> Monad for IO<A> {
    fn punt<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        f(self.value)
    }
}
