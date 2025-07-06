//mod inner;
mod switcher;
mod series;
mod pair;

pub use switcher::*;
pub use series::*;
pub use pair::*;

/// A trait for types that can be used as a parameter in [`HasChildren::get_child`](crate::HasChildren::get_child).
/// 
/// Whenever a struct implements [`HasChildren<'a, Primitive, Child>`](crate::HasChildren), [`get_child(path)`](crate::HasChildren::get_child) can be called on that struct with some `path: Primitive` to get a [Result] whose [Ok] value contains an object implementing `Child`
/// 
/// All types implementing [PathPrimitive] automatically implement [Path] 
pub trait PathPrimitive: Clone {}
impl PathPrimitive for () {}
impl <P: PathPrimitive> Path for P {}

/// A trait for types that can be used as a parameter in [`HasChildren::get_descendant`](crate::HasDescendants::get_descendant).
/// 
/// Whenever a struct implements [`HasDescendants<'a, DescendantPath, Joiner, Descendant>`](crate::HasDescendants), [`get_descendant(path)`](crate::HasDescendants::get_descendant) can be called on that struct with some `path: Path` to get a [Result] whose [Ok] value contains an object implementing `Descendant`
pub trait Path: Clone {
    fn pair_append<R: Path>(self, other: R) -> PathPair<Self,R> { PathPair::new(self,other) }
    fn pair_prepend<L: Path>(self, other: L) -> PathPair<L,Self> { PathPair::new(other,self) }
}

impl <S: Path> Into<PathSeries<S>> for PathPair<S,S> {
    fn into(self) -> PathSeries<S> { PathSeries::new([self.left,self.right]) }
}
impl <S: Path> Into<PathSeries<S>> for PathPair<S,PathSeries<S>> {
    fn into(mut self) -> PathSeries<S> { self.right.prepend(self.left); self.right }
}
impl <S: Path> Into<PathSeries<S>> for PathPair<PathSeries<S>,S> {
    fn into(mut self) -> PathSeries<S> { self.left.append(self.right); self.left }
}
impl <S: Path> Into<PathSeries<S>> for PathPair<PathSeries<S>,PathSeries<S>> {
    fn into(self) -> PathSeries<S> { PathSeries::new([self.left.into_paths(),self.right.into_paths()].concat()) }
}
