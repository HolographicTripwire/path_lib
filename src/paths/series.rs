use crate::paths::{Path};

pub struct PathSeries<P: Path>(Vec<P>);

impl <P:Path> PathSeries<P> where {
    pub fn empty() -> Self { Self(vec![]) }
    pub fn new<I: Into<P>>(series: impl IntoIterator<Item=I>) -> Self 
        { Self(series.into_iter().map(|p| p.into()).collect()) }
    
    pub fn prepend(&mut self, item: impl Into<P>) -> &Self { self.0.insert(0,item.into()); self }
    pub fn append(&mut self, item: impl Into<P>) -> &Self { self.0.push(item.into()); self }
    pub fn pop(&mut self) -> Option<P> { self.0.pop() }

    pub fn paths(&self) -> &Vec<P> { &self.0 }
    pub fn into_paths(self) -> Vec<P> { self.0 }
}

impl <P:Path, IP:Into<P>, It: IntoIterator<Item=IP>> From<It> for PathSeries<P> {
    fn from(value: It) -> Self { Self::new(value.into_iter().map(|v| v.into())) }
}

impl <P:Path> Path for PathSeries<P> {}

impl <P:Path> Clone for PathSeries<P> where {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl <P: Path + PartialEq> PartialEq for PathSeries<P> {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
} impl <P: Path + Eq> Eq for PathSeries<P> {}
