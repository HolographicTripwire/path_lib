use crate::paths::{Path};

/// A [Path] made of two subpaths (which are also [Paths](Path))
pub struct PathPair<L:Path, R:Path>{ pub left: L, pub right: R }

impl <L:Path, R:Path> PathPair<L,R> {
    pub fn new(left: L, right: R) -> Self { Self{left, right} }
} impl <L:Path, R:Path> Path for PathPair<L,R> {}

/// Implement common traits for [PathPair]
mod implement_common_traits {
    use super::*;

    // Implement Clone
    impl <L:Path, R:Path> Clone for PathPair<L,R> {
        fn clone(&self) -> Self { Self::new(self.left.clone(), self.right.clone()) }
    }

    // Implement PartialEq and Eq
    impl <L: Path + PartialEq, R: Path + PartialEq> PartialEq for PathPair<L,R> {
        fn eq(&self, other: &Self) -> bool { self.left == other.left && self.right == other.right }
    } impl <L: Path + Eq, R: Path + Eq> Eq for PathPair<L,R> {}

    // Implement Debug
    impl <L: Path + std::fmt::Debug, R: Path + std::fmt::Debug> std::fmt::Debug for PathPair<L,R> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PathPair").field("left", &self.left).field("right", &self.right).finish()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;

        #[test]
        fn test_clone() {
            let to_clone = PathPair::new("5", 5);
            let cloned = to_clone.clone();
            assert_eq!(to_clone.left, cloned.left);
            assert_eq!(to_clone.right, cloned.right);
        }

        #[test]
        fn test_eq_with_equal() {
            let left = PathPair::new("5", 5);
            let right = PathPair::new("5", 5);
            assert_eq!(left, right)
        }

        #[test]
        fn test_eq_with_inequal() {
            let left = PathPair::new("5", 5);
            let right = PathPair::new("5",4);
            assert_ne!(left, right)
        }
        
        #[test]
        fn test_debug() {
            let left = PathPair::new("5", 5);
            assert_eq!(format!("{:#?}", left), "PathPair {\n    left: \"5\",\n    right: 5,\n}")
        }
    }
}

/// Implement [From<T>](From) for PathPair for a variety of T
mod from {
    use super::*;

    // Tuple to pair: (L,R) -> PathPair<L,R>
    impl <L:Path, IL:Into<L>, R:Path, IR:Into<R>> From<(IL,IR)> for PathPair<L,R> {
        fn from(value: (IL,IR)) -> Self { Self::new(value.0.into(),value.1.into()) }
    }
}
/// Implement [Into<T>](Into) on [PathPair] for a variety of T
mod into {
    use crate::paths::PathSeries;

    use super::*;
    
    impl <S: Path> PathPair<S,S> {
        /// Convert this [PathPair<S,S>] to a [PathSeries\<S>](PathSeries)
        /// 
        /// E.g. PathPair(1,5) -> PathSeries([1,5])
        pub fn into_series(self) -> PathSeries<S> { PathSeries::<S>::new([self.left,self.right]) }
    } impl <S: Path> Into<PathSeries<S>> for PathPair<S,S> { fn into(self) -> PathSeries<S> { self.into_series() } }

    impl <S: Path> PathPair<S,PathSeries<S>> {
        /// Convert this [PathPair<S,PathSeries\<S>>] to a [PathSeries\<S>](PathSeries)
        /// 
        /// E.g. PathPair(1,\[5,2]) -> PathSeries(\[1,5,2])
        pub fn into_series(mut self) -> PathSeries<S> { self.right.prepend(self.left); self.right }
    } impl <S: Path> Into<PathSeries<S>> for PathPair<S,PathSeries<S>> { fn into(self) -> PathSeries<S> { self.into_series() } }

    impl <S: Path> PathPair<PathSeries<S>,S> {
        /// Convert this [PathPair<PathSeries\<S>,S>] to a [PathSeries\<S>](PathSeries)
        /// 
        /// E.g. PathPair(\[1,5],2) -> PathSeries(\[1,5,2])
        pub fn into_series(mut self) -> PathSeries<S> { self.left.append(self.right); self.left }
    } impl <S: Path> Into<PathSeries<S>> for PathPair<PathSeries<S>,S> { fn into(self) -> PathSeries<S> { self.into_series() } }

    impl <S: Path> PathPair<PathSeries<S>,PathSeries<S>> {
        /// Convert this [PathPair<PathSeries\<S>,PathSeries\<S>>] to a [PathSeries\<S>](PathSeries)
        /// 
        /// E.g. PathPair(\[1,5],\[2,7]) -> PathSeries(\[1,5,2,7])
        pub fn into_joined_series(self) -> PathSeries<S> { PathSeries::<S>::new([self.left.into_paths(),self.right.into_paths()].concat()) }
    } impl <S: Path> Into<PathSeries<S>> for PathPair<PathSeries<S>,PathSeries<S>> { fn into(self) -> PathSeries<S> { self.into_joined_series() } }
}
