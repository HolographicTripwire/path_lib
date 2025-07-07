/// Implements [PathUnit]
mod unit;
/// Implements [PathSeries]
mod series;
/// Implements [PathPair]
mod pair;

pub use unit::*;
pub use series::*;
pub use pair::*;

/// A trait for types that can be used as a parameter in [`HasChildren::get_child`](crate::HasChildren::get_child).
/// 
/// Whenever a struct implements [`HasChildren<'a, Primitive, Child>`](crate::HasChildren), [`get_child(path)`](crate::HasChildren::get_child) can be called on that struct with some `path: Primitive` to get a [Result] whose [Ok] value contains an object implementing `Child`
/// 
/// All types implementing [PathPrimitive] automatically implement [Path] 
pub trait PathPrimitive: Clone {}
impl <P: PathPrimitive> Path for P {}
impl PathPrimitive for () {}

/// A trait for types that can be used as a parameter in [`HasChildren::get_descendant`](crate::HasDescendants::get_descendant).
/// 
/// Whenever a struct implements [`HasDescendants<'a, DescendantPath, Joiner, Descendant>`](crate::HasDescendants), [`get_descendant(path)`](crate::HasDescendants::get_descendant) can be called on that struct with some `path: Path` to get a [Result] whose [Ok] value contains an object implementing `Descendant`
pub trait Path: Clone {
    /// Create a new [PathPair] with this as the left path and some other as the right
    fn pair_append<R: Path>(self, other: R) -> PathPair<Self,R> { PathPair::new(self,other) }
    /// Create a new [PathPair] with this as the right path and some other as the left
    fn pair_prepend<L: Path>(self, other: L) -> PathPair<L,Self> { PathPair::new(other,self) }
    
    /// Create a new [PathSeries] object with this as the only parameter
    fn into_unary_series(self) -> PathSeries<Self> { PathSeries::new([self]) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_append() {
        let pair = 5.pair_append("Hello");
        assert_eq!(pair.left, 5);
        assert_eq!(pair.right, "Hello");
    }

    #[test]
    fn test_pair_prepend() {
        let pair = 5.pair_prepend("Hello");
        assert_eq!(pair.left, "Hello");
        assert_eq!(pair.right, 5);
    }
}