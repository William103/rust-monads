pub trait Functor {
    type Wrapped<B>: Functor<Unwrapped = B>;
    type Unwrapped;

    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B;
}

impl<A> Functor for Option<A> {
    type Unwrapped = A;
    type Wrapped<B> = Option<B>;

    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(A) -> B,
    {
        match self {
            Self::Some(x) => Self::Wrapped::Some(f(x)),
            Self::None => Self::Wrapped::None,
        }
    }
}

impl<A, E> Functor for Result<A, E> {
    type Wrapped<B> = Result<B, E>;
    type Unwrapped = A;

    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
        self.map(f)
    }
}
