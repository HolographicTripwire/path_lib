use crate::paths::{Path};

pub struct PathSeries<P: Path>(Vec<P>);

impl <P:Path> PathSeries<P> where {
    pub fn new(series: Vec<P>) -> Self { Self(series) }
    pub fn paths(&self) -> &Vec<P> { &self.0 }
    pub fn into_paths(self) -> Vec<P> { self.0 }
}

impl <P:Path> From<Vec<P>> for PathSeries<P> {
    fn from(value: Vec<P>) -> Self { Self::new(value) }
}

impl <P:Path> Path for PathSeries<P> {}

impl <P:Path> Clone for PathSeries<P> where {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}
