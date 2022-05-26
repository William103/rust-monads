#[derive(Debug, PartialEq, Eq)]
pub enum List<A> {
    Null,
    Cons(A, Box<List<A>>),
}

#[macro_export]
macro_rules! list {
    [] => {
        List::Null
    };

    [ $x:expr ] => {
        List::Cons($x, Box::new(List::Null))
    };

    [ $x:expr, $( $xs:expr ),* ] => {
        List::Cons($x, Box::new(list![$( $xs ),*]))
    };
}

use List::*;

use crate::{
    applicative::Applicative, functor::Functor, monad::Monad, monoid::Monoid, semigroup::Semigroup,
};
impl<A> List<A> {
    pub fn zip_with<F, B, C>(self, other: List<B>, f: F) -> List<C>
    where
        F: Fn(A, B) -> C,
    {
        match (self, other) {
            (_, Null) => Null,
            (Null, _) => Null,
            (Cons(x, xs), Cons(y, ys)) => Cons(f(x, y), Box::new(xs.zip_with(*ys, f))),
        }
    }

    pub fn zip<B>(self, other: List<B>) -> List<(A, B)> {
        fn pair<A, B>(x: A, y: B) -> (A, B) {
            (x, y)
        }
        self.zip_with(other, pair)
    }

    pub fn foldr<F, B>(self, f: &F, init: B) -> B
    where
        F: Fn(A, B) -> B,
    {
        match self {
            Null => init,
            Cons(x, xs) => f(x, xs.foldr(f, init)),
        }
    }

    pub fn foldl<F, B>(self, f: &F, init: B) -> B
    where
        F: Fn(B, A) -> B,
    {
        match self {
            Null => init,
            Cons(x, xs) => xs.foldl(f, f(init, x)),
        }
    }

    pub fn filter<F>(self, f: F) -> Self
    where
        F: Fn(&A) -> bool,
    {
        self.foldr(
            &|x, acc| {
                if f(&x) {
                    Cons(x, Box::new(acc))
                } else {
                    acc
                }
            },
            Null,
        )
    }

    pub fn reverse(self) -> Self {
        self.foldl(&|acc, x| Cons(x, Box::new(acc)), Null)
    }

    // could be done with fold[rl], but making length consume `self`, while fine in Haskell, is
    // a bit weird in Rust
    pub fn length(&self) -> usize {
        match self {
            Null => 0,
            Cons(_x, xs) => 1 + xs.length(),
        }
    }

    pub fn partition<F>(self, f: F) -> (Self, Self)
    where
        F: Fn(&A) -> bool,
    {
        // rip TCO, rust doesn't do it anyway, so oh well
        self.foldr(
            &|x, (acc1, acc2)| {
                if f(&x) {
                    (Cons(x, Box::new(acc1)), acc2)
                } else {
                    (acc1, Cons(x, Box::new(acc2)))
                }
            },
            (Null, Null),
        )
    }

    pub fn take(self, n: usize) -> Self {
        match (self, n) {
            (_, 0) => Null,
            (Null, _) => Null,
            (Cons(x, xs), n) => Cons(x, Box::new(xs.take(n - 1))),
        }
    }

    pub fn ldrop(self, n: usize) -> Self {
        match (self, n) {
            (ls, 0) => ls,
            (Null, _) => Null,
            (Cons(_x, xs), n) => xs.ldrop(n - 1),
        }
    }
}

impl<A: Ord> List<A> {
    pub fn sort(self) -> Self {
        match self {
            Null => Null,
            Cons(p, xs) => {
                let (less, greater) = xs.partition(|x| x < &p);
                less.sort().op(Cons(p, Box::new(greater.sort())))
            }
        }
    }
}

impl<A> Functor for List<A> {
    type Unwrapped = A;
    type Wrapped<B> = List<B>;

    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(A) -> B,
    {
        match self {
            Self::Null => Self::Wrapped::Null,
            Self::Cons(x, xs) => Self::Wrapped::Cons(f(x), xs.map(|x| x.map(&f))),
        }
    }
}

impl<A> Applicative for List<A> {
    fn pure(x: A) -> Self {
        list![x]
    }

    // I still don't quite understand lift_a2 or <*>, but this seems reasonable to me
    fn lift_a2<F, B, C>(self, other: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: Fn(A, B) -> C,
    {
        self.zip_with(other, f)
    }
}

impl<A> Monad for List<A> {
    fn punt<F, B>(self, f: F) -> List<B>
    where
        F: Fn(A) -> List<B>,
    {
        Monoid::concat(self.map(f))
    }
}

impl<A> Semigroup for List<A> {
    fn op(self, y: List<A>) -> List<A> {
        match self {
            Null => y,
            Cons(x, xs) => Cons(x, Box::new(Self::op(*xs, y))),
        }
    }
}

impl<A> Monoid for List<A> {
    fn identity() -> List<A> {
        List::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lists() {
        use List::*;
        let ls = list![1, 2, 3];
        assert_eq!(
            ls,
            Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Null))))))
        );

        assert_eq!(ls.zip(list![1, 2, 3, 4]), list![(1, 1), (2, 2), (3, 3)]);

        let ls = list![1, 2, 3];
        assert_eq!(ls.zip_with(list![1, 2, 3, 4], |x, y| x + y), list![2, 4, 6]);

        let ls = list![1, 2, 3];
        assert_eq!(ls.map(|x| x * 2), list![2, 4, 6]);

        let ls = list![1, 2, 3];
        assert_eq!(ls.foldl(&|x, y| x + y, 0), 6);

        let ls = list![1, 2, 3];
        assert_eq!(ls.foldr(&|x, y| x + y, 0), 6);

        let ls = list![1, 2, 3];
        assert_eq!(ls.reverse(), list![3, 2, 1]);

        let ls = list![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(ls.filter(|x| x % 2 == 0), list![2, 4, 6, 8, 10]);

        use crate::mdo;

        // who even needs filter when you have monads, lol
        let ls = mdo! {
            val <= list![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            if val % 2 == 0 {
                list![val]
            } else {
                list![]
            }
        };
        assert_eq!(ls, list![2, 4, 6, 8, 10]);

        let ls = mdo! {
            val1 <= list![1, 2];
            val2 <= list![3, 4];
            return (val1, val2);
        };
        assert_eq!(ls, list![(1, 3), (1, 4), (2, 3), (2, 4)]);

        let ls = list![1, 2, 3, 4, 5, 6];
        assert_eq!(
            ls.partition(|x| x % 2 == 0),
            (list![2, 4, 6], list![1, 3, 5])
        );

        let ls = list![4, 1, 5, 2, 3, 7, 6];
        assert_eq!(ls.sort(), list![1, 2, 3, 4, 5, 6, 7]);
    }
}
