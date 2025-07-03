use crate::paths::{Path};

pub struct PathPair<L:Path, R:Path>(L,R); 

impl <L:Path, R:Path> PathPair<L,R> {
    pub fn new(left: L, right: R) -> Self { Self(left, right) }
    pub fn left(&self) -> &L { &self.0 }
    pub fn right(&self) -> &R { &self.1 }
}

impl <L:Path, IL:Into<L>, R:Path, IR:Into<R>> From<(IL,IR)> for PathPair<L,R> {
    fn from(value: (IL,IR)) -> Self { Self::new(value.0.into(),value.1.into()) }
}

impl <L:Path, R:Path> Path for PathPair<L,R> {}

impl <L:Path, R:Path> Clone for PathPair<L,R> {
    fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
}
