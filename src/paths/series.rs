use crate::paths::{Path};

pub struct PathSeries<P: Path>(Vec<P>);

impl <P:Path> PathSeries<P> where {
    pub fn new(series: Vec<P>) -> Self { Self(series) }
    pub fn paths(&self) -> &Vec<P> { &self.0 }
    pub fn into_paths(self) -> Vec<P> { self.0 }
}

impl <P:Path, IP:Into<P>, It: IntoIterator<Item=IP>> From<It> for PathSeries<P> {
    fn from(value: It) -> Self { Self::new(value.into_iter().map(|v| v.into()).collect()) }
}

impl <P:Path> Path for PathSeries<P> {}

impl <P:Path> Clone for PathSeries<P> where {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}
