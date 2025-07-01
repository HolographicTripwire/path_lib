//mod inner;
mod atomic;
mod switcher;
mod wrapper;
mod series;
mod joiner;

pub use atomic::*;
pub use switcher::*;
pub use wrapper::*;
pub use series::*;
pub use joiner::*;

pub trait PathPrimitive {}
impl PathPrimitive for () {}

pub enum PathImpl<L,R> {
    Switcher,
    Wrapper(L),
    Series(Vec<PathImpl<L,()>>),
    Joiner(Box<PathImpl<L,()>>,Box<PathImpl<R,()>>)
}
impl PathImpl<(),()> {
    fn switcher() -> Self { Self::Switcher }
}

impl <L: PathPrimitive> PathImpl<L,()> {
    fn atom(atom: L) -> Self { Self::Wrapper(atom) }
}
impl <L,R> PathImpl<PathImpl<L,R>,()> {
    fn wrapper(wrapped: PathImpl<L,R>) -> Self
        { Self::Wrapper(wrapped) }
}
impl <L> PathImpl<L,()> {
    fn series(series: Vec<PathImpl<L,()>>) -> Self
        { PathImpl::Series(series) }
}
impl <L,R> PathImpl<L,R> {
    fn joiner(left: PathImpl<L,()>, right: PathImpl<R,()>) -> Self
        { Self::Joiner(Box::new(left), Box::new(right)) }
}

pub trait Path<L,R>: Into<PathImpl<L,R>> {}
