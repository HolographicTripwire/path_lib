//mod inner;
mod atomic;
mod switcher;
mod series;
mod pair;

pub use atomic::*;
pub use switcher::*;
pub use series::*;
pub use pair::*;

pub trait PathPrimitive: Clone {}
impl PathPrimitive for () {}

pub trait Path: Clone {}

pub trait JoinablePath<P>: Path where P: Path {
    type Output;

    fn join(self, path: P) -> Self::Output;
}

impl <L, R> JoinablePath<R> for L where L: Path, R: Path {
    type Output = PathPair<L, R>;

    fn join(self, other: R) -> PathPair<L,R> {
        PathPair::new(self, other)
    }
}
