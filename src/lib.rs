#![feature(generic_associated_types)]

pub trait Semigroup {
    fn op(self, y: Self) -> Self;
}

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

pub trait Functor {
    type Wrapped<B>: Functor<Unwrapped = B>;
    type Unwrapped;

    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B;
}

pub trait Applicative: Functor {
    fn pure(x: Self::Unwrapped) -> Self;
    fn lift_a2<F, B, C>(self, other: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: Fn(Self::Unwrapped, B) -> C;
}

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

#[derive(Debug, PartialEq, Eq)]
pub enum List<A: Sized> {
    Null,
    Cons(A, Box<List<A>>),
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

impl<A, E> Monad for Result<A, E> {
    fn punt<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        self.and_then(f)
    }
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

impl<A> List<A> {
    pub fn zip_with<F, B, C>(self, other: List<B>, f: F) -> List<C>
    where
        F: Fn(A, B) -> C,
    {
        use List::*;
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
        use List::*;
        match self {
            Null => init,
            Cons(x, xs) => f(x, xs.foldr(f, init)),
        }
    }

    pub fn foldl<F, B>(self, f: &F, init: B) -> B
    where
        F: Fn(B, A) -> B,
    {
        use List::*;
        match self {
            Null => init,
            Cons(x, xs) => xs.foldl(f, f(init, x)),
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
            Self::Cons(x, xs) => Self::Wrapped::Cons(f(x), Box::new(xs.map(f))),
        }
    }
}

impl<A> Applicative for List<A> {
    fn pure(x: A) -> Self {
        list![x]
    }

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
        use List::*;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maybe() {
        assert_eq!(Some(3).map(|x| x * x), Some(9));
        assert_eq!(None.map(|x: usize| x + 3), None);
        assert_eq!(
            Some(3).lift_a2(Some(5), |x: i32, y: i32| -> i32 { x + y }),
            Some(8)
        );
        assert_eq!(Some(3).punt(|x| Some(x + 2)), Some(5));
        assert_eq!(None.punt(|x: i32| Some(x + 1)), None);

        let x: Option<i32> = mdo!(return 3;);
        assert_eq!(x, Some(3));

        let x: Option<i32> = mdo!(
            x <= Some(3);
            return x;
        );
        assert_eq!(x, Some(3));

        let div = |x: i32, y: i32| {
            if y == 0 {
                None
            } else {
                Some(x / y)
            }
        };
        let x: Option<i32> = mdo!(
            x <= Some(3);
            y <= Some(0);
            z <= div(x, y);
            return z;
        );
        assert_eq!(x, None);
    }

    #[test]
    fn test_list() {
        use List::*;
        let ls = list![1, 2, 3];
        // testing macro
        assert_eq!(
            ls,
            Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Null))))))
        );

        // testing map (fmap from Haskell)
        assert_eq!(ls.map(|x| x * x), list![1, 4, 9]);

        // testing Semigroup op
        let ls = list![1, 2, 3];
        let ls2 = list![4, 5, 6];
        assert_eq!(ls.op(ls2), list![1, 2, 3, 4, 5, 6]);

        // testing Monoid concat
        let lss = list![list![1, 2], list![3, 4], list![5, 6]];
        assert_eq!(List::concat(lss), list![1, 2, 3, 4, 5, 6]);

        // testing Applicative lift
        assert_eq!(
            list![1, 2, 3].lift_a2(list![4, 5, 6], |x, y| x + y),
            list![5, 7, 9]
        );

        // testing Monad
        assert_eq!(list![1, 2, 3].punt(|x| List::pure(x + 3)), list![4, 5, 6]);
    }

    #[test]
    fn test_writer() {
        // testing Functor
        assert_eq!(Writer::pure(3).map(|x| x + 1), Writer::pure(4),);

        // testing Applicative
        assert_eq!(
            Writer::pure(3).lift_a2(Writer::pure(5), |x, y| x + y),
            Writer::pure(8),
        );

        // testing Monad (this is the good bit)
        let double = |x| Writer::new(x * 2, &format!("doubling {}!\n", x));
        let half = |x| Writer::new(x / 2, &format!("halving {}!\n", x));
        let add = |x:i32 , y:i32| Writer::new(x + y, &format!("adding {} to {}!\n", x, y));
        assert_eq!(
            Writer::pure(2).punt(double).punt(double).punt(half),
            Writer::new(4, "doubling 2!\ndoubling 4!\nhalving 8!\n")
        );

        // testing Monad (with do-notation)
        let x = mdo! {
            x <= double(2);
            y <= Writer::pure(1);
            x <= add(x, y);
            x <= double(x);
            x <= half(x);
            return x;
        };
        assert_eq!(
            x,
            Writer::new(
                5,
                "doubling 2!\nadding 4 to 1!\ndoubling 5!\nhalving 10!\n"
            )
        );
    }
}
