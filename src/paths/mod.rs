//mod inner;
mod atomic;
mod switcher;
mod wrapper;
mod series;
mod pair;

pub use atomic::*;
pub use switcher::*;
pub use wrapper::*;
pub use series::*;
pub use pair::*;

pub trait PathPrimitive: Clone {}
impl PathPrimitive for () {}

#[derive(Clone)]
pub enum PathImpl<L: Clone,R: Clone> {
    Switcher,
    Wrapper(L),
    Series(Vec<PathImpl<L,()>>),
    Pair(Box<PathImpl<L,()>>,Box<PathImpl<R,()>>)
}
impl <L: Clone,R: Clone> Path<L,R> for PathImpl<L,R> {}

impl PathImpl<(),()> {
    fn switcher() -> Self { Self::Switcher }
}

impl <L: PathPrimitive> PathImpl<L,()> {
    fn atom(atom: L) -> Self { Self::Wrapper(atom) }
}
impl <L: Clone,R: Clone> PathImpl<PathImpl<L,R>,()> {
    fn wrapper(wrapped: PathImpl<L,R>) -> Self
        { Self::Wrapper(wrapped) }
}
impl <L: Clone> PathImpl<L,()> {
    fn series(series: Vec<PathImpl<L,()>>) -> Self
        { PathImpl::Series(series) }
}
impl <L: Clone,R: Clone> PathImpl<L,R> {
    fn pair(left: PathImpl<L,()>, right: PathImpl<R,()>) -> Self
        { Self::Pair(Box::new(left), Box::new(right)) }
}

pub trait Path<L: Clone,R: Clone>: Clone + Into<PathImpl<L,R>> {}
