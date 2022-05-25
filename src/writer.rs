use crate::{applicative::Applicative, functor::Functor, monad::Monad};

#[derive(Debug, PartialEq, Eq)]
pub struct Writer<A> {
    log: String,
    val: A,
}

impl<A> Writer<A> {
    pub fn new(val: A, log: &str) -> Writer<A> {
        Writer {
            log: log.to_string(),
            val,
        }
    }
}

impl<A> Functor for Writer<A> {
    type Unwrapped = A;
    type Wrapped<B> = Writer<B>;

    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(A) -> B,
    {
        Writer {
            log: self.log,
            val: f(self.val),
        }
    }
}

impl<A> Applicative for Writer<A> {
    fn pure(x: A) -> Self {
        Writer {
            log: String::new(),
            val: x,
        }
    }

    fn lift_a2<F, B, C>(self, other: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: Fn(A, B) -> C,
    {
        Writer {
            log: self.log + &other.log,
            val: f(self.val, other.val),
        }
    }
}

impl<A> Monad for Writer<A> {
    fn punt<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(A) -> Self::Wrapped<B>,
    {
        let Writer {
            log: update,
            val: new_val,
        } = f(self.val);
        Writer {
            log: self.log + &update,
            val: new_val,
        }
    }
}
