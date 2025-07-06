use crate::paths::{Path};

pub struct PathPair<L:Path, R:Path>{ pub left: L, pub right: R }

impl <L:Path, R:Path> PathPair<L,R> {
    pub fn new(left: L, right: R) -> Self { Self{left, right} }
} impl <L:Path, R:Path> Path for PathPair<L,R> {}

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
            assert_eq!(to_clone.left, cloned.left)
        }
    }
}

mod from {
    use super::*;

    impl <L:Path, IL:Into<L>, R:Path, IR:Into<R>> From<(IL,IR)> for PathPair<L,R> {
        fn from(value: (IL,IR)) -> Self { Self::new(value.0.into(),value.1.into()) }
    }
}
mod into {
    use crate::paths::PathSeries;

    use super::*;

    impl <L:Path, R:Path> Into<PathSeries<PathPair<L,R>>> for PathPair<L,R> {
        fn into(self) -> PathSeries<PathPair<L,R>> { PathSeries::new([self]) }
    }
    
    impl <S: Path> Into<PathSeries<S>> for PathPair<S,S> {
        fn into(self) -> PathSeries<S> { PathSeries::<S>::new([self.left,self.right]) }
    }
    impl <S: Path> Into<PathSeries<S>> for PathPair<S,PathSeries<S>> {
        fn into(mut self) -> PathSeries<S> { self.right.prepend(self.left); self.right }
    }
    impl <S: Path> Into<PathSeries<S>> for PathPair<PathSeries<S>,S> {
        fn into(mut self) -> PathSeries<S> { self.left.append(self.right); self.left }
    }
    impl <S: Path> Into<PathSeries<S>> for PathPair<PathSeries<S>,PathSeries<S>> {
        fn into(self) -> PathSeries<S> { PathSeries::<S>::new([self.left.into_paths(),self.right.into_paths()].concat()) }
    }    
}
