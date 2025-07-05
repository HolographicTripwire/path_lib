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
pub trait Path: Clone {}

/// A trait for [Path] objects that can be joined with specific, other types of [Path] objects, to produce some new [Path] object [Output](JoinablePath::Output)
/// 
/// Currently all [Path] objects `L` implenent [JoinablePath<R>], with an [Output](JoinablePath::Output) of [PathPair<L,R>](PathPair).
/// 
/// Future updates may have:
/// - [`PathPrimitive`].join([`PathPrimitive`]) -> [`PathSeries<PathPrimitive>`](PathSeries)
/// - [`PathPrimitive`].join([`PathSeries<PathPrimitive>`](PathSeries)) -> [`PathSeries<PathPrimitive>`](PathSeries)
/// - [`PathSeries<PathPrimitive>`](PathSeries).join([`PathPrimitive`]) -> [`PathSeries<PathPrimitive>`](PathSeries)
/// - [`PathSeries<PathPrimitive>`](PathSeries).join([`PathSeries<PathPrimitive>`](PathSeries)) -> [`PathSeries<PathPrimitive>`](PathSeries)
pub trait AppendablePath<P>: Path where P: Path {
    /// The [Path] produced by joining this path to `P`
    type Output: Path;

    /// Join this path with another to produce a path of type [Output](JoinablePath::Output)
    fn append(self, path: P) -> Self::Output;
}

impl <L, R> AppendablePath<R> for L where L: Path, R: Path {
    type Output = PathPair<L, R>;

    fn append(self, other: R) -> PathPair<L,R>
        { PathPair::new(self, other) }
}

/// An inverse version of [AppendablePath]
pub trait PrependablePath<P>: Path where P: Path {
    /// The [Path] produced by joining this path to `P`
    type Output: Path;

    /// Join this path with another to produce a path of type [Output](JoinablePath::Output)
    fn prepend(self, path: P) -> Self::Output;
}

impl <L, R, O: Path> PrependablePath<L> for R where 
L: AppendablePath<R,Output=O>, R: Path  {
    type Output = O;
    fn prepend(self, path: L) -> Self::Output
        { path.append(self) }
}
