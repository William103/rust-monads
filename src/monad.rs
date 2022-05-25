use crate::applicative::Applicative;

pub trait Monad: Applicative {
    fn punt<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> Self::Wrapped<B>;

    fn and_then<B>(&self, other: Self::Wrapped<B>) -> Self::Wrapped<B> {
        other
    }
}

#[macro_export]
macro_rules! mdo {
    { return $x:expr; } => {
        Applicative::pure($x)
    };

    { $x:ident <= $ex:expr; $( $xs:tt )* } => {
        $ex.punt(|$x| {
            mdo!($( $xs )*)
        })
    };

    { $x:expr; $( $xs:tt )* } => {
        $x.and_then(mdo!($( $xs )*))
    };

    { $x:stmt; $( $xs:tt )* } => {
        {
            $x;
            mdo!($( $xs )*)
        }
    };

    { $x:expr } => {
        $x
    };
}

impl<A> Monad for Option<A> {
    fn punt<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(A) -> Self::Wrapped<B>,
    {
        use Option::*;
        match self {
            Some(x) => f(x),
            None => None,
        }
    }
}

impl<A, E> Monad for Result<A, E> {
    fn punt<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        self.and_then(f)
    }
}
