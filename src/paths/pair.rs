use crate::paths::{Path};

pub struct PathPair<L:Path, R:Path>{ pub left: L, pub right: R }

impl <L:Path, R:Path> PathPair<L,R> {
    pub fn new(left: L, right: R) -> Self { Self{left, right} }
}

impl <L:Path, IL:Into<L>, R:Path, IR:Into<R>> From<(IL,IR)> for PathPair<L,R> {
    fn from(value: (IL,IR)) -> Self { Self::new(value.0.into(),value.1.into()) }
}

impl <L:Path, R:Path> Path for PathPair<L,R> {}

impl <L:Path, R:Path> Clone for PathPair<L,R> {
    fn clone(&self) -> Self { Self::new(self.left.clone(), self.right.clone()) }
}

impl <L: Path + PartialEq, R: Path + PartialEq> PartialEq for PathPair<L,R> {
    fn eq(&self, other: &Self) -> bool { self.left == other.left && self.right == other.right }
} impl <L: Path + Eq, R: Path + Eq> Eq for PathPair<L,R> {}

impl <L: Path + std::fmt::Debug, R: Path + std::fmt::Debug> std::fmt::Debug for PathPair<L,R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathPair").field("left", &self.left).field("right", &self.right).finish()
    }
}
